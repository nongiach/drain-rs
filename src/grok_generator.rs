use crate::log_cluster::LogCluster;
use grok;
use grok::Pattern;
use std::collections::HashMap;

// a build.rs generates patterns.rs at compilation time
// it contains all GROK base patterns loaded here
include!(concat!(env!("OUT_DIR"), "/patterns.rs"));

/// Return the default GROK base patterns.
pub fn base_patterns<'a>() -> &'a [(&'a str, &'a str)] {
    PATTERNS
}

/// stores all compiled patterns at one place
#[derive(Debug)]
pub struct GrokGenerator {
    grok: grok::Grok,                            // the grok processor
    compiled_patterns: HashMap<String, Pattern>, // all grok compiled pattern pattern_name: Pattern
}

impl GrokGenerator {
    /// generates a GrokGenerator initialized with the base patterns from the patterns folder
    pub fn new_with_base_patterns() -> Self {
        let mut grok = grok::Grok::empty();
        let mut compiled_patterns = HashMap::new();

        // load base patterns
        for &(name, pattern_definition) in base_patterns() {
            grok.insert_definition(name, pattern_definition);
        }
        // compile base patterns
        for &(name, _) in base_patterns() {
            // compiled_patterns.push(grok.compile("%{USERNAME}", false).unwrap());
            let grok_pattern_name = format!("%{{{}}}", name);
            compiled_patterns.insert(
                grok_pattern_name.to_owned(),
                grok.compile(&grok_pattern_name, false).unwrap(),
            );
        }

        GrokGenerator {
            grok,
            compiled_patterns,
        }
    }

    /// calculates the pattern score against several strings
    /// the score is the count of string matching this pattern
    fn pattern_score_against_string_vector(
        &self,
        pattern: &Pattern,
        strings: &Vec<String>,
    ) -> usize {
        strings
            .iter()
            .filter(|string| match pattern.match_against(string) {
                Some(matches) => matches.len() > 0,
                None => false,
            })
            .count()
    }

    /// takes as input a list of strings and detect the best base GROK pattern for it
    pub fn detect_grok_for_a_list_of_string(&self, strings: &Vec<String>) -> Option<String> {
        let mut best_match = None;
        let mut best_match_score = 0;
        for (pattern_name, compiled_patterns) in self.compiled_patterns.iter() {
            let current_score =
                self.pattern_score_against_string_vector(compiled_patterns, strings);
            if current_score > best_match_score {
                best_match_score = current_score;
                best_match = Some(pattern_name.to_owned());
                // println!(
                //     "[New Match score] {:?} => {} => {:?}",
                //     strings, best_match_score, pattern_name
                // );
            }
        }
        best_match
    }

    /// takes a log cluster a generate a GROK line pattern from it
    #[allow(dead_code)]
    pub fn generate_grok_from_logcluster(_logcluster: LogCluster) -> Option<String> {
        Some("test".to_string())
    }
}
