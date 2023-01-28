//genetic pool

use rand;
use rand::distributions::Distribution;
pub mod gene;
pub mod player;

const BOARD_SIZE: i64 = 8 * 8;

#[derive(Clone)]
pub struct Pool {
    pub genes: Vec<gene::Gene>,
    pub players: Vec<player::Player>,
}

impl Pool {
    pub fn new(pop_size: usize) -> Pool {

        //input to output connections

        let mut initial: Vec<gene::Gene> = Vec::new();

        for i in 0..BOARD_SIZE {
            initial.push(gene::Gene::new(i, -1));
        }

        let mut players: Vec<player::Player> = Vec::new();

        let sampler = rand::distributions::Uniform::<f64>::new(-1.0, 1.0);
        let mut rng = rand::thread_rng();

        for i in 0..pop_size {
            let mut p = player::Player::new();

            for j in &initial {
                p.add_gene(*j, sampler.sample(&mut rng));
            }

            players.push(p);
        }

        return Pool{
            genes: initial,
            players: players,
        };
    }

    pub fn fitness(player: &player::Player) -> f64 {
        return player.wins as f64 + 0.5 * player.draws as f64;
    }

    pub fn cross(&self, p1: player::Player, p2: player::Player) -> player::Player {
        let mut crossed = player::Player::new();
        return crossed;
    }
}