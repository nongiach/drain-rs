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

// takes as input a list of strings and generate a GROK pattern from it
#[allow(dead_code)]
pub fn detect_grok_for_a_list_of_string(_strings: &Vec<String>) -> Option<String> {
    Some("test".to_string())
}

// takes a log cluster a generate a GROK line  pattern from it
#[allow(dead_code)]
pub fn generate_grok_from_logcluster(_logcluster: LogCluster) -> Option<String> {
    Some("test".to_string())
}

pub struct GrokGenerator {
    grok: grok::Grok,
    compiled_patterns: Vec<Pattern>,
}

impl GrokGenerator {
    pub fn new() -> Self {
        let mut grok = grok::Grok::empty();
        let mut compiled_patterns = Vec::<Pattern>::new();

        // load base patterns
        for &(name, pattern_definition) in base_patterns() {
            grok.insert_definition(name, pattern_definition);
        }
        // compile base patterns
        for &(name, _) in base_patterns() {
            // compiled_patterns.push(grok.compile("%{USERNAME}", false).unwrap());
            let grok_pattern_name = format!("%{{{}}}", name);
            compiled_patterns.push(grok.compile(&grok_pattern_name, false).unwrap());
            println!("grok_pattern_name");
        }

        GrokGenerator {
            grok,
            compiled_patterns,
        }
    }
}
