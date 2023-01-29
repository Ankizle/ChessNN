mod pool;
mod game;

const POP_SIZE: usize = 4;
const DEPTH: i64 = 2;

fn run(pool: &mut pool::Pool, generation: u64) {

    println!("Generation {}", generation);

    let mut wins = Vec::<usize>::new();
    let mut loses = Vec::<usize>::new();
    let mut draws = Vec::<usize>::new();

    for (k, i) in pool.players.iter().enumerate() {
        for (l, j) in pool.players.iter().enumerate() {
            if i as *const _ == j as *const _ {
                continue;
            }
            
            let res = game::play_game(i, j, DEPTH);

            if res == 0 {
                //draw
                draws.push(k);
                draws.push(l);
            } else if res == 1 {
                //white wins
                wins.push(k);
                loses.push(l);
            } else if res == 2 {
                //black wins
                loses.push(k);
                wins.push(l);
            }
        }
    }

    for i in wins {
        pool.players[i].win();
    }
    for i in loses {
        pool.players[i].lose();
    }
    for i in draws {
        pool.players[i].draw();
    }

    pool.next_generation();
    run(pool, generation + 1);
}

fn main() {
    let mut pool = pool::Pool::new(POP_SIZE);
    run(&mut pool, 0);
}
