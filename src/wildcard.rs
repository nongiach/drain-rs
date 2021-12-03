// use serde::{Deserialize, Deserializer, Serialize, Serializer};
// use std::collections::HashSet;

// // A wildcard is used to say that this token can be of any values
// // For the purpose of the algorithm we store all encountered values
// // so they can be analyzed to propose the correct grok pattern
// #[derive(Debug, Serialize, Deserialize)]
// pub struct WildCard {
//     collected_values: HashSet<String>,
// }

// impl WildCard {
//     pub fn new() -> Self {
//         WildCard {
//             collected_values: HashSet::new(),
//         }
//     }
// }
