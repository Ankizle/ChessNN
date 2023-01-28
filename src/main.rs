use chess;

mod pool;
mod game;

const POP_SIZE: usize = 10;
const DEPTH: i64 = 5;

fn main() {

    let mut players: Vec<pool::player::Player> = Vec::new();
    let mut pool = pool::Pool::new();

    for i in 0..POP_SIZE {
        players.push(pool::player::Player::new(&pool));   
    }

    let mut ii: i64 = 0;

    for i in &players {
        for j in &players {
            if i as *const _ == j as *const _ {
                continue;
            }
            game::play_game(i, j, DEPTH);

            println!("{}", ii);
            ii += 1;
        }
    }
}
