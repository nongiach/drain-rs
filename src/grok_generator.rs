use crate::log_cluster::LogCluster;

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
