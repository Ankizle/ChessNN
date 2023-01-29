//genetic pool

use rand;
use rand::distributions::Distribution;
use rand::Rng;
use std::collections::HashSet;
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

        for _i in 0..pop_size {
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

    pub fn get_fit(&self) -> Vec<&player::Player> {
        //get sqrt(pop_size) most fit players

        let mut fit_values = Vec::<(&player::Player, f64)>::new();

        for p in &self.players {
            fit_values.push((p, Pool::fitness(&p)));
        }

        fit_values.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap()); //sort greatest to least
        let mut fit_players: Vec<&player::Player> = fit_values.into_iter().map(|x| x.0).rev().collect();
        fit_players.truncate((self.players.len() as f64).sqrt() as usize);

        return fit_players;
    }

    pub fn cross(&self, p1: &player::Player, p2: &player::Player) -> player::Player {
        let mut crossed = player::Player::new();

        let mut shared = HashSet::<gene::Gene>::new();
        let mut disjoint = Vec::<(&player::Player, gene::Gene)>::new();

        for i in &p1.genes {
            if p2.genes.contains_key(i.0) {
                shared.insert(*i.0);
            } else {
                disjoint.push((&p1, *i.0));
            }
        }

        for i in &p2.genes {
            disjoint.push((&p2, *i.0));
        }

        let mut rng = rand::thread_rng();

        for i in shared {

            let mut lo = p1.genes[&i];
            let mut hi = p2.genes[&i];

            if lo > hi {
                (hi, lo) = (lo, hi);
            }

            //very unlikely
            if lo == hi {
                crossed.add_gene(i, lo);
                continue;
            }

            let between = rand::distributions::Uniform::<f64>::new(lo, hi);
            crossed.add_gene(i, between.sample(&mut rng));
        }

        let hi_fit = Pool::fitness(p1).max(Pool::fitness(p2));

        for i in disjoint {
            if hi_fit <= Pool::fitness(i.0) {
                crossed.add_gene(i.1, i.0.genes[&i.1]);
                continue;
            }

            //if its not the more fit parent
            //half chance of getting their disjoint gene

            if rng.gen_range(0..1) == 0 {
                crossed.add_gene(i.1, i.0.genes[&i.1]);
            }
        }

        return crossed;
    }

    pub fn next_generation(&mut self) {
        let fit = self.get_fit();

        let mut bred = Vec::<player::Player>::new();

        for i in &fit {
            for j in &fit {
                bred.push(self.cross(i, j));
            }
        }

        self.players = bred;
    }
}