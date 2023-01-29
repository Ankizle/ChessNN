//player

use crate::pool::gene;
use std::collections::HashMap;
use chess;

#[derive(Debug, Clone)]
pub struct Player {
    pub network: Vec<Vec<(i64, f64) /* output node, weight */> /* every index represents the input node */>,
    pub genes: HashMap<gene::Gene, f64> /* gene and weight */,
    pub wins: i64,
    pub draws: i64,
    pub loses: i64,
}

impl Player {

    pub fn new() -> Player {
        return Player{
            network: Vec::new(),
            genes: HashMap::new(),
            wins: 0,
            draws: 0,
            loses: 0,
        };
    }

    pub fn add_gene(&mut self, gene: gene::Gene, weight: f64) {
        self.genes.insert(gene, weight);

        while gene.in_node as usize >= self.network.len() {
            self.network.push(Vec::new());
        }

        self.network[gene.in_node as usize].push((gene.out_node, weight));
    }

    fn board_to_num(board: &chess::Board) -> Vec<f64> {

        let mut tiles: Vec<f64> = Vec::new();

        for i in 0..64 {
            unsafe {
                let s = chess::Square::new(i);
                let p = board.piece_on(s);

                match p {
                    None => tiles.push(0.0),
                    Some(i) => tiles.push(i.to_index() as f64 + 1.0),
                }
            }
        }

        return tiles;
    }

    pub fn eval_gene(&self, i: i64, val: &f64) -> f64 {

        if i == -1 {
            return *val;
        }

        if self.network.len() <= i as usize {
            return 0.0;
        }

        let conns = &self.network[i as usize];
        let mut sum = 0.0;

        for c in conns {
            let nv = val * c.1;
            sum += self.eval_gene(c.0, &nv)
        }

        return sum;
    }

    pub fn eval(&self, board: &chess::Board) -> f64 {

        let inputs = Player::board_to_num(board);

        let mut out = 0.0;

        for (k, v) in inputs.iter().enumerate() {
            out += self.eval_gene(k as i64, v); 
        }

        return out;
    }

    pub fn draw(&mut self) {
        self.draws += 1;
    }

    pub fn win(&mut self) {
        self.wins += 1;
    }

    pub fn lose(&mut self) {
        self.loses += 1;
    }
}