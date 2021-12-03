use crate::token::Token;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize)]
/// Represents a cluster of logs
pub struct LogCluster {
    // The tokens representing this unique cluster
    log_tokens: Vec<Token>,
    // The number logs matched
    num_matched: u64,
}

impl fmt::Display for LogCluster {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, count [{}] ",
            self.log_tokens
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            self.num_matched
        )
    }
}

impl LogCluster {
    pub fn new(log_tokens: Vec<Token>) -> LogCluster {
        LogCluster {
            log_tokens,
            num_matched: 1,
        }
    }

    /// How many logs have been matched in this cluster
    pub fn num_matched(&self) -> u64 {
        self.num_matched
    }

    /// Grab the current token strings
    pub fn as_string(&self) -> String {
        self.log_tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Grab the current detailed token strings
    pub fn as_detailed_string(&self) -> String {
        self.log_tokens
            .iter()
            .map(|t| t.as_detailed_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn similarity(&self, log: &[Token]) -> GroupSimilarity {
        let len = self.log_tokens.len() as f32;
        let mut approximate_similarity: u32 = 0;
        let mut exact_similarity: f32 = 0.0;

        for (pattern, token) in self.log_tokens.iter().zip(log.iter()) {
            if token == pattern {
                exact_similarity += 1.0;
            } else if (*pattern).is_wildcard() {
                approximate_similarity += 1;
            }
        }
        GroupSimilarity {
            approximate_similarity,
            exact_similarity: exact_similarity / len,
        }
    }

    pub fn add_log(&mut self, new_log: &[Token]) -> &LogCluster {
        // update log cluster if we detect variable parts
        for (new_token, stored_token) in new_log.iter().zip(self.log_tokens.iter_mut()) {
            if !new_token.is_wildcard() {
                // check if the current log_line token is the different from the logcluster tokens
                if new_token != stored_token && !stored_token.is_wildcard() {
                    *stored_token = Token::new_empty_wildcard();
                }
                stored_token.add_token_if_wildcard(&new_token);
            }
        }
        self.num_matched += 1;
        self
    }
    pub fn extract_variables(&self, log: &[Token]) -> Vec<String> {
        // Extract values of the variable parts into a hashmap
        let mut variables = vec![];
        for (i, token) in log.iter().enumerate() {
            if self.log_tokens[i].is_wildcard() {
                // println!("{} -> {}", token, self.log_tokens[i].as_string_detailed());
                variables.push(token.to_string());
            }
        }
        variables
    }
}

#[derive(PartialEq, Debug)]
pub struct GroupSimilarity {
    approximate_similarity: u32,
    pub exact_similarity: f32,
}

impl core::cmp::PartialOrd for GroupSimilarity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.exact_similarity.partial_cmp(&other.exact_similarity) {
            Some(order) => match order {
                Ordering::Equal => self
                    .approximate_similarity
                    .partial_cmp(&other.approximate_similarity),
                Ordering::Less => Some(Ordering::Less),
                Ordering::Greater => Some(Ordering::Greater),
            },
            None => None,
        }
    }
}
