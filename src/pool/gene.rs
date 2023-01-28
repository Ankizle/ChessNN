//gene

static mut INNOVATION_COUNTER: i64 = 0;

#[derive(Copy, Clone)]
pub struct Gene {
    pub in_node: i64,
    pub out_node: i64, //-1 for final output
    innovation_number: i64,
}

impl Gene {
    pub fn new(in_node: i64, out_node: i64) -> Gene {
        unsafe {
            INNOVATION_COUNTER += 1;
            return Gene{
                in_node: in_node,
                out_node: out_node,
                innovation_number: INNOVATION_COUNTER,
            };
        }
    }
}
