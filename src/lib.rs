mod piece_tables;

use std::{collections::HashMap, thread, vec::Vec};

use shakmaty::{Bitboard, Chess, Color, File, Move, Outcome, Piece, Position, Role, Square};
use crate::piece_tables::{BISHOPS, BLACK_KING_MG, BLACK_ROOKS, KING_EG, KINGSIDE_BLACK_PAWNS, KINGSIDE_FILES, KINGSIDE_WHITE_PAWNS, KNIGHTS, QUEENS, QUEENSIDE_BLACK_PAWNS, QUEENSIDE_WHITE_PAWNS, UNCASTLED_BLACK_PAWNS, UNCASTLED_WHITE_PAWNS, WHITE_KING_MG, WHITE_ROOKS};


fn analyze_pawns(our_bitboard: &Bitboard, their_bitboard: &Bitboard, color: &Color) -> (i32, i32, i32) {
    let mut doubled_pawns = 0;
    let mut isolated_pawns = 0;
    let mut passed_pawns = 0;

    for file in File::ALL {
        let file_bb = Bitboard::from_file(file);
        let file_pawns: Bitboard = *our_bitboard & file_bb;

        if file_pawns.count() == 0 {
            continue;
        }

        if file_pawns.count() > 1 {
            doubled_pawns += file_pawns.count() as i32;
        }

        let neighbor_files = match file {
            File::A => Bitboard::from_file(File::B),
            File::H => Bitboard::from_file(File::G),
            _ => Bitboard::from_file(File::new(file as u32 - 1)) | Bitboard::from_file(File::new(file as u32 + 1)),
        };

        if (neighbor_files & *our_bitboard).is_empty() {
            isolated_pawns += 1;
        }

        let forward_files: Bitboard = match file {
            File::A => Bitboard::from_file(File::A) | Bitboard::from_file(File::B),
            File::H => Bitboard::from_file(File::H) | Bitboard::from_file(File::G),
            _ => Bitboard::from_file(file) | Bitboard::from_file(File::new(file as u32 - 1)) | Bitboard::from_file(File::new(file as u32 + 1)),
        };

        let mut forward_squares = forward_files;
        for _ in 0..6 {
            match color {
                Color::Black => forward_squares = forward_squares.shift(-8),
                Color::White => forward_squares = forward_squares.shift(8),
            }
        }

        if (forward_squares & *their_bitboard).is_empty() {
            passed_pawns += file_pawns.count() as i32;
        }
    }

    return (doubled_pawns, isolated_pawns, passed_pawns);
}


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

    let mut score = 0f64;

    let our_material = board.board().material_side(board.turn()).iter().sum::<u8>() as f64;
    let their_material = board.board().material_side(!board.turn()).iter().sum::<u8>() as f64;
    score += our_material - their_material;

    score += board.legal_moves().len() as f64 * 0.05;

    let our_pawns = board.board().by_piece(Piece{role: Role::Pawn, color: board.turn()});
    let their_pawns = board.board().by_piece(Piece{role: Role::Pawn, color: !board.turn()});
    let (our_doubled, our_isolated, our_passed) = analyze_pawns(&our_pawns, &their_pawns, &board.turn());
    let (their_doubled, their_isolated, their_passed) = analyze_pawns(&their_pawns, &our_pawns, &!board.turn());

    score -= our_doubled as f64 * 0.1;
    score -= our_isolated as f64 * 0.05;

    score += their_doubled as f64 * 0.1;
    score += their_isolated as f64 * 0.05;

    score += (our_passed * 2) as f64;
    score -= (their_passed * 2) as f64;

    let castled = board.castles().is_empty();
    let white_king_file = board.board().by_piece(Piece{role: Role::King, color: Color::White}).into_iter().next().unwrap().file();
    let black_king_file = board.board().by_piece(Piece{role: Role::King, color: Color::Black}).into_iter().next().unwrap().file();
    for sq in Square::ALL {
        if let Some(piece) = board.board().piece_at(sq) {
            let piece_value = match piece.role {
                Role::King => {
                    if our_material + their_material < 30f64 {
                        return KING_EG[sq.rank() as usize][sq.file() as usize] as f64;
                    }
                    return match piece.color {
                        Color::White => WHITE_KING_MG[sq.rank() as usize][sq.file() as usize] as f64,
                        Color::Black => BLACK_KING_MG[sq.rank() as usize][sq.file() as usize] as f64,
                    };
                },
                Role::Queen => QUEENS[sq.rank() as usize][sq.file() as usize] as f64,
                Role::Bishop => BISHOPS[sq.rank() as usize][sq.file() as usize] as f64,
                Role::Knight => KNIGHTS[sq.rank() as usize][sq.file() as usize] as f64,
                Role::Rook => {
                    return match piece.color {
                        Color::White => WHITE_ROOKS[sq.rank() as usize][sq.file() as usize] as f64,
                        Color::Black => BLACK_ROOKS[sq.rank() as usize][sq.file() as usize] as f64,
                    };
                },
                Role::Pawn => {
                    // If the king cannot castle anymore, use the castled pawn configurations
                    if castled {
                        let our_king_file = match piece.color {
                            Color::White => white_king_file,
                            Color::Black => black_king_file,
                        };
                        // If the king is not on the kingside files, use the queenside positions
                        if (Bitboard::from_file(our_king_file) & KINGSIDE_FILES).is_empty() {
                            return match piece.color {
                                Color::White => QUEENSIDE_WHITE_PAWNS[sq.rank() as usize][sq.file() as usize] as f64,
                                Color::Black => QUEENSIDE_BLACK_PAWNS[sq.rank() as usize][sq.file() as usize] as f64,
                            };
                        }
                        return match piece.color {
                            Color::White => KINGSIDE_WHITE_PAWNS[sq.rank() as usize][sq.file() as usize] as f64,
                            Color::Black => KINGSIDE_BLACK_PAWNS[sq.rank() as usize][sq.file() as usize] as f64,
                        };
                    }

                    return match piece.color {
                        Color::White => UNCASTLED_WHITE_PAWNS[sq.rank() as usize][sq.file() as usize] as f64,
                        Color::Black => UNCASTLED_BLACK_PAWNS[sq.rank() as usize][sq.file() as usize] as f64,
                    };
                }
            };

            if piece.color == board.turn() {
                score += piece_value as f64 * 0.1;
            } else {
                score -= piece_value as f64 * 0.1;
            }
        }
    }

    return score;
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
