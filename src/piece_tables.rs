#![allow(unused)]

use shakmaty::Bitboard;

pub const UNCASTLED_WHITE_PAWNS: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [0, 5, 15, 25, 25, 15, 5, 0],
    [0, 0, 10, 20, 20, 10, 0, 0],
    [5, -5, 5, 0, 0, -5, -5, 5],
    [5, 5, 0, -20, -20, 5, 5, 5],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

pub const KINGSIDE_WHITE_PAWNS: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [0, 5, 15, 25, 25, 5, 0, 0],
    [0, 5, 10, 20, 20, -5, -10, 0],
    [5, 5, 5, 0, 0, -10, -10, 10],
    [0, -5, -5, -20, -20, 10, 10, 10],
    [0, 0, 0, 0, 0, 0, 0, 0],
];

pub const QUEENSIDE_WHITE_PAWNS: [[i32; 8]; 8] = [
    [KINGSIDE_WHITE_PAWNS[0][7], KINGSIDE_WHITE_PAWNS[0][6], KINGSIDE_WHITE_PAWNS[0][5], KINGSIDE_WHITE_PAWNS[0][4], KINGSIDE_WHITE_PAWNS[0][3], KINGSIDE_WHITE_PAWNS[0][2], KINGSIDE_WHITE_PAWNS[0][1], KINGSIDE_WHITE_PAWNS[0][0]],
    [KINGSIDE_WHITE_PAWNS[1][7], KINGSIDE_WHITE_PAWNS[1][6], KINGSIDE_WHITE_PAWNS[1][5], KINGSIDE_WHITE_PAWNS[1][4], KINGSIDE_WHITE_PAWNS[1][3], KINGSIDE_WHITE_PAWNS[1][2], KINGSIDE_WHITE_PAWNS[1][1], KINGSIDE_WHITE_PAWNS[1][0]],
    [KINGSIDE_WHITE_PAWNS[2][7], KINGSIDE_WHITE_PAWNS[2][6], KINGSIDE_WHITE_PAWNS[2][5], KINGSIDE_WHITE_PAWNS[2][4], KINGSIDE_WHITE_PAWNS[2][3], KINGSIDE_WHITE_PAWNS[2][2], KINGSIDE_WHITE_PAWNS[2][1], KINGSIDE_WHITE_PAWNS[2][0]],
    [KINGSIDE_WHITE_PAWNS[3][7], KINGSIDE_WHITE_PAWNS[3][6], KINGSIDE_WHITE_PAWNS[3][5], KINGSIDE_WHITE_PAWNS[3][4], KINGSIDE_WHITE_PAWNS[3][3], KINGSIDE_WHITE_PAWNS[3][2], KINGSIDE_WHITE_PAWNS[3][1], KINGSIDE_WHITE_PAWNS[3][0]],
    [KINGSIDE_WHITE_PAWNS[4][7], KINGSIDE_WHITE_PAWNS[4][6], KINGSIDE_WHITE_PAWNS[4][5], KINGSIDE_WHITE_PAWNS[4][4], KINGSIDE_WHITE_PAWNS[4][3], KINGSIDE_WHITE_PAWNS[4][2], KINGSIDE_WHITE_PAWNS[4][1], KINGSIDE_WHITE_PAWNS[4][0]],
    [KINGSIDE_WHITE_PAWNS[5][7], KINGSIDE_WHITE_PAWNS[5][6], KINGSIDE_WHITE_PAWNS[5][5], KINGSIDE_WHITE_PAWNS[5][4], KINGSIDE_WHITE_PAWNS[5][3], KINGSIDE_WHITE_PAWNS[5][2], KINGSIDE_WHITE_PAWNS[5][1], KINGSIDE_WHITE_PAWNS[5][0]],
    [KINGSIDE_WHITE_PAWNS[6][7], KINGSIDE_WHITE_PAWNS[6][6], KINGSIDE_WHITE_PAWNS[6][5], KINGSIDE_WHITE_PAWNS[6][4], KINGSIDE_WHITE_PAWNS[6][3], KINGSIDE_WHITE_PAWNS[6][2], KINGSIDE_WHITE_PAWNS[6][1], KINGSIDE_WHITE_PAWNS[6][0]],
    [KINGSIDE_WHITE_PAWNS[7][7], KINGSIDE_WHITE_PAWNS[7][6], KINGSIDE_WHITE_PAWNS[7][5], KINGSIDE_WHITE_PAWNS[7][4], KINGSIDE_WHITE_PAWNS[7][3], KINGSIDE_WHITE_PAWNS[7][2], KINGSIDE_WHITE_PAWNS[7][1], KINGSIDE_WHITE_PAWNS[7][0]],
];

pub const UNCASTLED_BLACK_PAWNS: [[i32; 8]; 8] = [
    UNCASTLED_WHITE_PAWNS[7],
    UNCASTLED_WHITE_PAWNS[6],
    UNCASTLED_WHITE_PAWNS[5],
    UNCASTLED_WHITE_PAWNS[4],
    UNCASTLED_WHITE_PAWNS[3],
    UNCASTLED_WHITE_PAWNS[2],
    UNCASTLED_WHITE_PAWNS[1],
    UNCASTLED_WHITE_PAWNS[0],
];

pub const KINGSIDE_BLACK_PAWNS: [[i32; 8]; 8] = [
    KINGSIDE_WHITE_PAWNS[7],
    KINGSIDE_WHITE_PAWNS[6],
    KINGSIDE_WHITE_PAWNS[5],
    KINGSIDE_WHITE_PAWNS[4],
    KINGSIDE_WHITE_PAWNS[3],
    KINGSIDE_WHITE_PAWNS[2],
    KINGSIDE_WHITE_PAWNS[1],
    KINGSIDE_WHITE_PAWNS[0],
];

pub const QUEENSIDE_BLACK_PAWNS: [[i32; 8]; 8] = [
    QUEENSIDE_WHITE_PAWNS[7],
    QUEENSIDE_WHITE_PAWNS[6],
    QUEENSIDE_WHITE_PAWNS[5],
    QUEENSIDE_WHITE_PAWNS[4],
    QUEENSIDE_WHITE_PAWNS[3],
    QUEENSIDE_WHITE_PAWNS[2],
    QUEENSIDE_WHITE_PAWNS[1],
    QUEENSIDE_WHITE_PAWNS[0],
];

pub const KNIGHTS: [[i32; 8]; 8] = [
    [-50, -40, -30, -30, -30, -30, -40, -50],
    [-40, -20, 0, 5, 5, 0, -20, -40],
    [-30, 0, 15, 15, 15, 15, 5, -30],
    [-30, 5, 20, 25, 25, 20, 5, -30],
    [-30, 5, 20, 25, 25, 20, 5, -30],
    [-30, 5, 15, 15, 15, 15, 5, -30],
    [-40, -20, 0, 5, 5, 0, -20, -40],
    [-50, -40, -30, -30, -30, -30, -40, -50],
];

pub const BISHOPS: [[i32; 8]; 8] = [
    [0, -5, -5, -10, -10, -5, -5, -10],
    [-10, 20, 0, 0, 0, 0, 20, -10],
    [-10, 0, 15, 5, 5, 15, 0, -10],
    [-10, 0, 5, 10, 10, 5, 0, -10],
    [-10, 0, 5, 10, 10, 5, 0, -10],
    [-10, 0, 15, 5, 5, 15, 0, -10],
    [-10, 20, 0, 0, 0, 0, 20, -10],
    [0, -5, -5, -10, -10, -5, -5, -10],
];

pub const WHITE_ROOKS: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [5, 10, 10, 10, 10, 10, 10, 5],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, -5, 5, 5, 5, 5, -5, -5],
];

pub const BLACK_ROOKS: [[i32; 8]; 8] = [
    WHITE_ROOKS[7],
    WHITE_ROOKS[6],
    WHITE_ROOKS[5],
    WHITE_ROOKS[4],
    WHITE_ROOKS[3],
    WHITE_ROOKS[2],
    WHITE_ROOKS[1],
    WHITE_ROOKS[0],
];

pub const QUEENS: [[i32; 8]; 8] = [
    [-20, -10, -5, 0, 0, -5, -10, -20],
    [-10, 0, 5, 0, 0, 5, 0, -10],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-10, 0, 5, 0, 0, 5, 0, -10],
    [-20, -10, -5, 0, 0, -5, -10, -20],
];

pub const WHITE_KING_MG: [[i32; 8]; 8] = [
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-20, -40, -40, -40, -40, -40, -40, -20],
    [-10, -20, -20, -10, -10, -20, -20, -10],
    [20, 20, -10, -5, -5, -10, -5, 20],
    [20, 30, 0, -5, -5, 0, 30, 20],
];

pub const BLACK_KING_MG: [[i32; 8]; 8] = [
    WHITE_KING_MG[7],
    WHITE_KING_MG[6],
    WHITE_KING_MG[5],
    WHITE_KING_MG[4],
    WHITE_KING_MG[3],
    WHITE_KING_MG[2],
    WHITE_KING_MG[1],
    WHITE_KING_MG[0],
];

pub const KING_EG: [[i32; 8]; 8] = [
    [-50, -30, -30, -30, -30, -30, -30, -50],
    [-30, -30, 5, 10, 10, 5, -30, -30],
    [-30, -10, 20, 30, 30, 20, -10, -30],
    [-30, -10, 30, 40, 40, 30, -10, -30],
    [-30, -10, 30, 40, 40, 30, -10, -30],
    [-30, -10, 20, 30, 30, 20, -10, -30],
    [-30, -30, 5, 10, 10, 5, -30, -30],
    [-50, -30, -30, -30, -30, -30, -30, -50],
];

pub const QUEENSIDE_FILES: Bitboard = Bitboard(0x0F0F0F0F0F0F0F0F);
pub const KINGSIDE_FILES: Bitboard = Bitboard(0xF0F0F0F0F0F0F0F0);
