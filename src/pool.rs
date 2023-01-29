//genetic pool

use rand;
use rand::distributions::Distribution;
use rand::Rng;
use std::collections::HashSet;
use std::collections::HashMap;
use rand::prelude::IteratorRandom;
pub mod gene;
pub mod player;

const BOARD_SIZE: i64 = 8 * 8;

#[derive(Clone)]
pub struct Pool {
    pub genes: Vec<gene::Gene>,
    pub players: Vec<player::Player>,
    mutation_chance: f32,
    generation_number: u32,
}

impl Pool {
    pub fn new(pop_size: usize, mutation_chance: f32) -> Pool {

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
            mutation_chance: mutation_chance,
            generation_number: 0,
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

    fn mutate(&self, mut genome: HashMap<gene::Gene, f64>) -> HashMap<gene::Gene, f64> {
        let mut rng = rand::thread_rng();
        let dist = rand::distributions::WeightedIndex::new(&[self.mutation_chance, 1.0 - self.mutation_chance]).unwrap();
        
        if dist.sample(&mut rng) == 1 {
            return genome;
        }

        let picked = rng.gen_range(0..3);

        //3 cases

        if picked == 0 {

            //create a new gene entirely

            let mut new_gene: gene::Gene;

            loop {

                let input = rng.gen_range(0..(self.genes.len() + 1)) as i64;
                let mut output = rng.gen_range(63..(self.genes.len() + 2)) as i64;

                if output == 63 { //output can also be the final output node
                    output = -1
                }

                new_gene = gene::Gene::new(input, output);

                if !genome.contains_key(&new_gene) {
                    break;
                }
            }

            let sampler = rand::distributions::Uniform::<f64>::new(-1.0, 1.0);

            genome.insert(new_gene, sampler.sample(&mut rng));
        } else if picked == 1 {

            //modify weight of an existing gene

            let chosen = genome.keys().choose(&mut rng).unwrap();
            let sampler = rand::distributions::Uniform::<f64>::new(-2.0, 2.0);

            genome.insert(*chosen, sampler.sample(&mut rng));

        } else {

            //remove gene

            let chosen = genome.keys().choose(&mut rng).unwrap();
            let mut n_genome = genome.clone();

            n_genome.remove(chosen);
            return n_genome;
        }

        return genome;
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

        let mut genome = HashMap::<gene::Gene, f64>::new();

        let mut rng = rand::thread_rng();

        for i in shared {

            let mut lo = p1.genes[&i];
            let mut hi = p2.genes[&i];

            if lo > hi {
                (hi, lo) = (lo, hi);
            }

            //very unlikely
            if lo == hi {
                genome.insert(i, lo);
                continue;
            }

            let between = rand::distributions::Uniform::<f64>::new(lo, hi);
            genome.insert(i, between.sample(&mut rng));
        }

        let hi_fit = Pool::fitness(p1).max(Pool::fitness(p2));

        for i in disjoint {
            if hi_fit <= Pool::fitness(i.0) {
                genome.insert(i.1, i.0.genes[&i.1]);
                continue;
            }

            //if its not the more fit parent
            //half chance of getting their disjoint gene

            if rng.gen_range(0..1) == 0 {
                genome.insert(i.1, i.0.genes[&i.1]);
            }
        }

        let mutated = self.mutate(genome);

        for (k, v) in mutated {
            crossed.add_gene(k, v);
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
        self.generation_number += 1;
    }
}