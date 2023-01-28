//genetic pool

pub mod gene;
pub mod player;

pub struct Pool {
    pub genes: Vec<gene::Gene>,
}

impl Pool {
    pub fn new() -> Pool {
        return Pool{
            genes: Vec::new(),
        };
    }
}