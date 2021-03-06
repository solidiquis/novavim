#![allow(dead_code)]
pub const BOLD: i64 = 1;
pub const UNDERLINE: i64 = BOLD << 1;
pub const STRIKETHROUGH: i64 = BOLD << 2;
pub const HIGHLIGHT: i64 = BOLD << 3;
pub const FG_BLACK: i64 = BOLD << 4;
pub const FG_RED: i64 = BOLD << 5;
pub const FG_GREEN: i64 = BOLD << 6;
pub const FG_YELLOW: i64 = BOLD << 7;
pub const FG_BLUE: i64 = BOLD << 8;
pub const FG_MAGENTA: i64 = BOLD << 9;
pub const FG_CYAN: i64 = BOLD << 10;
pub const FG_WHITE: i64 = BOLD << 11;
pub const BG_BLACK: i64 = BOLD << 12;
pub const BG_RED: i64 = BOLD << 13;
pub const BG_GREEN: i64 = BOLD << 14;
pub const BG_YELLOW: i64 = BOLD << 15;
pub const BG_BLUE: i64 = BOLD << 16;
pub const BG_MAGENTA: i64 = BOLD << 17;
pub const BG_CYAN: i64 = BOLD << 18;
pub const BG_WHITE: i64 = BOLD << 19;
