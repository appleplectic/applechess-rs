use std::{
    collections::HashMap,
    thread,
    vec::Vec
};

use shakmaty::{
    Chess,
    Move,
    Outcome,
    Position,
};


fn get_heuristic(board: &Chess) -> f64 {
    if let Some(outcome) = board.outcome() {
        if outcome == Outcome::Draw {
            return 0.01;
        }
        if let Some(winner) = outcome.winner() {
            if winner == board.turn() {
                return -1000000f64;
            }
            return 1000000f64;
        }
    }
    return board.board().material_side(board.turn()).iter().sum::<u8>() as f64 -
        board.board().material_side(!board.turn()).iter().sum::<u8>() as f64;
}


fn alphabeta(node: &Chess,
             depth: i32,
             maximizing_player: bool,
             mut a: f64,
             mut b: f64
) -> f64 {
    if depth == 0 || node.is_game_over() {
        return get_heuristic(&node);
    }

    if maximizing_player {
        let mut value = f64::NEG_INFINITY;
        for legal_move in node.legal_moves() {
            let mut child = node.clone();
            child.play_unchecked(&legal_move);
            value = f64::max(value, alphabeta(&child, depth-1, false, a, b));
            a = f64::max(a, value);
            if value >= b {
                break;
            }
        }
        return value;
    }

    let mut value = f64::INFINITY;
    for legal_move in node.legal_moves() {
        let mut child = node.clone();
        child.play_unchecked(&legal_move);
        value = f64::min(value, alphabeta(&child, depth-1, true, a, b));
        b = f64::min(b, value);
        if value <= a {
            break;
        }
    }
    return value;
}


pub fn get_best_move(board: &Chess, depth: i32) -> Move {
    let mut handles = Vec::new();

    for legal_move in board.legal_moves() {
        let mut moved_board = board.clone();
        let handle = thread::spawn(move || {
            moved_board.play_unchecked(&legal_move);
            return (legal_move, alphabeta(&moved_board, depth, false, f64::NEG_INFINITY, f64::INFINITY));
        });
        handles.push(handle);
    }

    let mut scores = HashMap::new();
    for handle in handles {
        let score = handle.join().expect("Couldn't join handle.");
        scores.insert(score.0, score.1);
    }

    return scores.iter().max_by(
        |a, b|
            (*a).1.partial_cmp((*b).1).expect("Couldn't compare f64s.")
    ).expect("Couldn't get maximum value.").0.to_owned();
}
