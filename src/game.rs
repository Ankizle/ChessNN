//simulate a chess game between two players

use crate::pool::player;
use chess;

/*
0 - draw
1 - white wins
2 - black wins
*/
pub fn play_game(white: &player::Player, black: &player::Player, depth: i64) -> i8 {
    let mut game = chess::Game::new_with_board(chess::Board::default());

    let mut turn = 1;

    while game.result().is_none() && !game.can_declare_draw() {
        let mv = evaluate(&mut game.current_position(), if turn == 1 {
            white
        } else {
            black
        }, true, depth, -f64::INFINITY, f64::INFINITY).1;

        match mv {
            None => panic!("Game ran into 0 moves without stalemate"),
            Some(i) => game.make_move(i),
        };

        turn *= -1;
    }

    return 
        match game.result() {
            None => 0,
            Some(r) => 
                if r == chess::GameResult::WhiteCheckmates { return 1; }
                else if r == chess::GameResult::BlackCheckmates { return 2; }
                else { return 0; },
        }
}

pub fn evaluate(board: &mut chess::Board, player: &player::Player, is_player: bool, depth: i64, p_alpha: f64, p_beta: f64) -> (f64, Option<chess::ChessMove>) {
    let is_player_si: f64 = if is_player { 1.0 } else { -1.0 };

    if board.status() == chess::BoardStatus::Checkmate {
        return (-is_player_si * f64::INFINITY, None);
    }

    if board.status() == chess::BoardStatus::Stalemate {
        return (0.0, None);
    }

    if depth == 0 {
        return (player.eval(board), None);
    }

    let move_gen = chess::MoveGen::new_legal(board);

    let mut best_eval = -is_player_si * f64::INFINITY;
    let mut best_eval_move: Option<chess::ChessMove> = None;

    let mut alpha = p_alpha;
    let mut beta = p_beta;

    for m in move_gen {
        let e = evaluate(&mut board.make_move_new(m), player, !is_player, depth - 1, alpha, beta).0;

        let cond = if is_player { e >= best_eval } else { e <= best_eval };
        
        if cond {
            best_eval = e;
            best_eval_move = Some(m);
        }

        if is_player && alpha < best_eval {
            alpha = best_eval;
        } else if !is_player && beta > best_eval {
            beta = best_eval;
        }

        if beta <= alpha {
            break;
        }
    }

    return (best_eval, best_eval_move);
}