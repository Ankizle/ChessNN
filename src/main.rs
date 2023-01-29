mod pool;
mod game;

use std::sync::{Arc, Mutex};
use std::fs;
use std::thread;

const POP_SIZE: usize = 4;
const DEPTH: i64 = 5;
const MUTATION_CHANCE: f32 = 0.70;

fn run(pool: &mut pool::Pool) {

    println!("Generation {}", pool.generation_number);

    let _ = fs::create_dir(format!("models/generation{}", generation));

    for (k, p) in pool.players.iter().enumerate() {
        let _ = p.save(&format!("models/generation{}/player{}.json", generation, k));
    }

    let mut game_num = 1;

    let mut thread_pool = vec![];

    let wins = Arc::new(Mutex::new(vec![]));
    let loses = Arc::new(Mutex::new(vec![]));
    let draws = Arc::new(Mutex::new(vec![]));

    for i in 0..(pool.players.len()) {
        for j in 0..(pool.players.len())  {

            if i == j {
                continue;
            }

            println!("Game {}", game_num);

            game_num += 1;

            let mut pool_cl_wh = pool.clone();
            let mut pool_cl_bl = pool.clone();

            let wins = Arc::clone(&wins);
            let loses = Arc::clone(&loses);
            let draws = Arc::clone(&draws);

            thread_pool.push(thread::spawn(move || {
                let res = game::play_game(&mut pool_cl_wh.players[i], &mut pool_cl_bl.players[j], DEPTH);

                let mut draws = draws.lock().unwrap();
                let mut wins = wins.lock().unwrap();
                let mut loses = loses.lock().unwrap();

                if res == 0 {
                    //draw
                    draws.push(i);
                    draws.push(j);
                } else if res == 1 {
                    //white wins
                    wins.push(i);
                    loses.push(j);
                } else if res == 2 {
                    //black wins
                    loses.push(i);
                    wins.push(j);
                }
            }));

        }
    }

    for i in thread_pool {
        let _ = i.join().unwrap();
    }

    let wins = wins.lock().unwrap();
    let loses = loses.lock().unwrap();
    let draws = draws.lock().unwrap();

    for i in 0..(wins.len()) {
        pool.players[*wins.get(i).unwrap()].win()
    }
    for i in 0..(loses.len()) {
        pool.players[*loses.get(i).unwrap()].lose()
    }
    for i in 0..(draws.len()) {
        pool.players[*draws.get(i).unwrap()].draw()
    }

    pool.next_generation();
    run(pool);
}

fn main() {
    let mut pool = pool::Pool::new(POP_SIZE, MUTATION_CHANCE);
    run(&mut pool);
}
