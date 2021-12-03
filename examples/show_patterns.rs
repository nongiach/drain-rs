extern crate drain_rs;
use grok;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::{fmt, fmt::Display, fmt::Formatter};

// impl<'a> grok::Matches<'a> {
//     pub fn as_string(self) {}
// }

// impl Display for grok::Matches {
//     // `f` is a buffer, and this method must write the formatted string into it
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "tata")
//     }
// }
//https://github.com/jordansissel/ruby-grok/blob/master/patterns/pure-ruby/base
include!(concat!(env!("OUT_DIR"), "/patterns.rs"));
/// Return the default patterns.
pub fn patterns<'a>() -> &'a [(&'a str, &'a str)] {
    PATTERNS
}

// show all stored patterns
pub fn main() {
    let mut grok = grok::Grok::with_patterns();
    for &(key, value) in patterns() {
        println!("{} => {}", key, value);
        let pattern = grok
            .compile(format!("%{{{}:value}}", key).as_str(), false)
            .expect("Error while compiling!");
        if let Some(matches) = pattern.match_against("<134>") {
            print!("{} => {} ;; ", matches.len(), matches.get("value").unwrap());

            // for (key, value) in matches.iter() {
            //     print!("{} => {};", key, value);
            // }
            // // println!("{}", matches);
            println!("{} => {}", key, value);
        }
        // break;
    }
}
