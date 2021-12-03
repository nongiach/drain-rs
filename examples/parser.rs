extern crate drain_rs;
use drain_rs::grok_generator::GrokGenerator;
use grok;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

/// Read in a formatted log and print out the log clusters
pub fn main() {
    let mut g = grok::Grok::with_patterns();
    let filter_patterns = vec![
        "blk_(|-)[0-9]+", //blockid
                          // "%{IPV4:ip_address}", //IP
                          // "%{NUMBER:number}",   //Num
                          // "\\s+",               //string
    ];
    // Build new drain tree
    let mut drain = drain_rs::DrainTree::new()
        .filter_patterns(filter_patterns)
        .max_depth(4)
        .max_children(100)
        .min_similarity(0.4)
        // HDFS log pattern, variable format printout in the content section
        // .log_pattern("%{NUMBER:date} %{NUMBER:time} %{NUMBER:proc} %{LOGLEVEL:level} %{DATA:component}: %{GREEDYDATA:content}", "content")
        // Compile all the grok patterns so that they can be used
        .build_patterns(&mut g);
    let filename = env::args()
        .nth(1)
        .expect("Missing required argument file name");
    let reader: Box<dyn BufRead> = Box::new(BufReader::new(fs::File::open(filename).unwrap()));
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            if let Some(_log_cluster) = drain.add_log_line(line.as_str()) {
                // println!("log_line    => {}", line);
                // println!("log_cluster => {}", log_cluster);
                // log_cluster.extract_variables()
            }
        }
    }
    // drain.log_groups().iter().for_each(|f| println!("{}", *f));
    // drain
    //     .log_groups()
    //     .iter()
    //     .for_each(|f| println!("{}", f.as_detailed_string()));

    // post processing
    // println!("{}", drain);
    let grok_generator = GrokGenerator::new_with_base_patterns();

    drain.log_groups().iter().for_each(|log_cluster| {
        println!("{}", log_cluster.as_detailed_string());
        log_cluster.detect_best_grok(&grok_generator);
    });
}
