use crate::log_cluster::LogCluster;
use grok;
use grok::Pattern;
// a build.rs generates patterns.rs at compilation time
// it contains all GROK base patterns loaded here
include!(concat!(env!("OUT_DIR"), "/patterns.rs"));

/// Return the default GROK base patterns.
pub fn base_patterns<'a>() -> &'a [(&'a str, &'a str)] {
    PATTERNS
}

/// Check if the pattern is a good one that we can trust
pub fn is_a_good_pattern<'a>(pattern: &'a str) -> bool {
    let good_patterns = vec![
        "PATH",
        "URI",
        "DATE",
        "MAC",
        "IP",
        "HOSTPORT",
        "USERNAME",
        "BASE16NUM",
        "SYSLOGTIMESTAMP",
        "QS",
        "HOSTNAME",
    ];
    if good_patterns.contains(&pattern) {
        true
    } else {
        false
    }
}

/// stores all compiled patterns at one place
#[derive(Debug)]
pub struct GrokGenerator {
    grok: grok::Grok,                                  // the grok processor
    compiled_patterns: Vec<(String, String, Pattern)>, // (pattern_name, grok_pattern, compiled_pattern)
}

impl GrokGenerator {
    /// generates a GrokGenerator initialized with the base patterns from the patterns folder
    pub fn new_with_base_patterns() -> Self {
        let mut grok = grok::Grok::empty();
        let mut compiled_patterns = Vec::new();

        // load base patterns
        for &(name, pattern_definition) in base_patterns() {
            grok.insert_definition(name, pattern_definition);
        }

        // compile base patterns
        for &(pattern_name, _) in base_patterns() {
            // compiled_patterns.push(grok.compile("%{USERNAME}", false).unwrap());
            if is_a_good_pattern(pattern_name) {
                let grok_pattern = format!("%{{{}}}", pattern_name);
                let compiled_pattern = grok
                    .compile(&format!("^{}.?$", grok_pattern), false)
                    .unwrap();
                compiled_patterns.push((pattern_name.to_owned(), grok_pattern, compiled_pattern));
            }
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
        pattern_name: &String,
        compiled_pattern: &Pattern,
        strings: &Vec<String>,
    ) -> usize {
        strings
            .iter()
            .filter(|string| match compiled_pattern.match_against(string) {
                Some(matches) => {
                    // println!(
                    //     "{} matches => {:?}",
                    //     pattern_name,
                    //     matches.get(pattern_name)
                    // );
                    matches.len() > 0
                }
                None => false,
            })
            .count()
    }

    /// takes as input a list of strings and detect the best base GROK pattern for it
    pub fn detect_grok_for_a_list_of_string(&self, strings: &Vec<String>) -> Option<String> {
        let mut best_match = None;
        let mut best_match_score = 0;
        for (pattern_name, grok_pattern, compiled_patterns) in self.compiled_patterns.iter() {
            let current_score =
                self.pattern_score_against_string_vector(pattern_name, compiled_patterns, strings);
            if current_score > best_match_score {
                best_match_score = current_score;
                best_match = Some(grok_pattern.to_owned());
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
