use crate::grok_generator::GrokGenerator;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeSet;
use std::fmt;
use std::fmt::Formatter;

// A wildcard is used to say that this token can be of any values
// For the purpose of the algorithm we store all encountered values
// so they can be analyzed to propose the correct grok pattern
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Token {
    WildCard(BTreeSet<String>),
    Val(String),
}

impl Token {
    pub fn new_wildcard(init_string: String) -> Self {
        let mut wildcard_values = BTreeSet::<String>::new();
        wildcard_values.insert(init_string);
        Token::WildCard(wildcard_values)
    }
    pub fn new_wildcard_from_token(init_token: Token) -> Self {
        match init_token {
            Token::Val(init_string) => Token::new_wildcard(init_string),
            _ => unreachable!("This code should never be reached"),
        }
    }
    pub fn new_empty_wildcard() -> Self {
        Token::WildCard(BTreeSet::new())
    }
    pub fn is_wildcard(&self) -> bool {
        match self {
            Self::WildCard(_) => true,
            Self::Val(_) => false,
        }
    }
    pub fn add_token_if_wildcard(&mut self, new_token: &Token) {
        // add a new token to the wildcard
        // this has no effect if this token (self) is not a wildcard
        // also the number of stored token is of maximum 10
        if let Token::WildCard(btreeset) = self {
            if btreeset.len() < 10 {
                btreeset.insert(new_token.to_string());
            }
        }
    }
    pub fn as_detailed_string(&self) -> String {
        match self {
            Token::Val(s) => format!("{}", s.as_str()),
            Token::WildCard(s) => {
                let vec = s.iter().collect::<Vec<&String>>();
                format!("<** {:?} **>", vec)
            }
        }
    }

    pub fn as_string_vector(&self) -> Option<Vec<String>> {
        match self {
            Token::Val(_) => None,
            Token::WildCard(s) => Some(s.clone().into_iter().collect::<Vec<String>>()),
        }
    }

    pub fn detect_best_grok(&self, grok_generator: &GrokGenerator) -> String {
        match self {
            Token::Val(constant_value) => constant_value.to_owned(),
            _ => match &self.as_string_vector() {
                Some(string_vector) => {
                    grok_generator.detect_grok_for_a_list_of_string(&string_vector)
                }
                None => None,
            }
            .unwrap_or("<unknown>".to_string()),
        }
    }
}

struct TokenVisitor;
impl<'de> Visitor<'de> for TokenVisitor {
    type Value = Token;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value == "<*>" {
            Ok(Token::new_empty_wildcard())
        } else {
            Ok(Token::Val(String::from(value)))
        }
    }
}

impl Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Token {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TokenVisitor)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Val(s) => write!(f, "{}", s.as_str()),
            Token::WildCard(_) => write!(f, "<*>"),
        }
    }
}

impl std::clone::Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::WildCard(s) => Token::WildCard(s.clone()),
            Token::Val(s) => Token::Val(s.clone()),
        }
    }
}
