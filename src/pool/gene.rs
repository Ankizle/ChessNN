//gene

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct Gene {
    pub in_node: i64,
    pub out_node: i64, //-1 for final output
}

impl Gene {
    pub fn new(in_node: i64, out_node: i64) -> Gene {
        return Gene{
            in_node: in_node,
            out_node: out_node,
        };
    }
}
