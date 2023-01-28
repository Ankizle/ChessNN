//player

use crate::pool;
use crate::pool::gene;
use chess;

const BOARD_SIZE: i64 = 8 * 8;

pub struct Player<'l> {
    pool: &'l pool::Pool,
    genes: Vec<gene::Gene>,
    wins: i64,
    draws: i64,
    loses: i64,
}

impl Player<'_> {

    pub fn new(pool: &pool::Pool) -> Player {
        return Player{
            pool: pool,
            genes: Vec::new(),
            wins: 0,
            draws: 0,
            loses: 0,
        };
    }

    pub fn eval(&self, board: &chess::Board) -> f64 {
        return 1.0;
    }
}