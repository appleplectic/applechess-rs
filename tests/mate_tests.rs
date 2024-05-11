use std::{fs, str::FromStr};
use regex::Regex;
use shakmaty::{san::San, fen::Fen, CastlingMode, Chess, FromSetup};
use applechess_rs::*;


fn fen_helper(fen: String, depth: i32) -> String {
    let board = Chess::from_setup(Fen::from_str(&fen).unwrap().0, CastlingMode::Standard).unwrap();
    let best_move = get_best_move(&board, depth);
    return San::from_move(&board, &best_move).to_string();
}


#[test]
fn m2_tests() {
    let file = fs::read_to_string("tests/m8n2.txt").unwrap();
    let re = Regex::new(r"((([1-8rbqnkpRBQNKP]+)/){7}([1-8rbqnkpRBQNKP]+) ([bw]) [kqKQ\-] ([a-h][1-8]|-) \d \d)\n1(.|...) (.*?)([#+])?").unwrap();

    for caps in re.captures_iter(&file) {
        let fen = caps.get(1).unwrap().as_str();
        let expected_result = caps.get(8).unwrap().as_str();

        assert_eq!(fen_helper(fen.to_string(), 2), expected_result.to_string());
    }
}

#[test]
fn m3_tests() {
    let file = fs::read_to_string("tests/m8n3.txt").unwrap();
    let re = Regex::new(r"((([1-8rbqnkpRBQNKP]+)/){7}([1-8rbqnkpRBQNKP]+) ([bw]) [kqKQ\-] ([a-h][1-8]|-) \d \d)\n1(.|...) (.*?)([#+])?").unwrap();

    for caps in re.captures_iter(&file) {
        let fen = caps.get(1).unwrap().as_str();
        let expected_result = caps.get(8).unwrap().as_str();

        assert_eq!(fen_helper(fen.to_string(), 3), expected_result.to_string());
    }
}

#[test]
fn m4_tests() {
    let file = fs::read_to_string("tests/m8n4.txt").unwrap();
    let re = Regex::new(r"((([1-8rbqnkpRBQNKP]+)/){7}([1-8rbqnkpRBQNKP]+) ([bw]) [kqKQ\-] ([a-h][1-8]|-) \d \d)\n1(.|...) (.*?)([#+])?").unwrap();

    for caps in re.captures_iter(&file) {
        let fen = caps.get(1).unwrap().as_str();
        let expected_result = caps.get(8).unwrap().as_str();

        assert_eq!(fen_helper(fen.to_string(), 4), expected_result.to_string());
    }
}
