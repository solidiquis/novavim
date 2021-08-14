pub mod constants;

use std::fmt::Display;
use std::io::{self, Write};

use constants::*;

macro_rules! flush_print {
    ( $($t:tt)* ) => {
        {
            use std::io;
            use std::io::Write;
            let mut h = io::stdout();
            write!(h, $($t)* ).unwrap();
            h.flush().unwrap();
        }
    }
}

pub fn echo(txt: &str) {
    flush_print!("{}", txt)
}

pub fn sgrecho(txt: &str, flags: i64) {
    sgr(flags);
    echo(txt);
    sgr_normal()
}

pub fn cur_up(n: usize) {
    flush_print!("{}", fansi(n, "A"))
}

pub fn cur_down(n: usize) {
    flush_print!("{}", fansi(n, "B"))
}

pub fn cur_right(n: usize) {
    flush_print!("{}", fansi(n, "C"))
}

pub fn cur_left(n: usize) {
    flush_print!("{}", fansi(n, "D"))
}

pub fn cur_home() {
    flush_print!("{}", fansi("H", ""))
}

pub fn cur_set_pos(row: usize, col: usize) {
    let pos = format!("{};{}", row, col);
    flush_print!("{}", fansi(&pos, "H"))
}

pub fn cur_save_pos() {
    flush_print!("{}", fansi("", "s"))
}

pub fn cur_restore_pos() {
    flush_print!("{}", fansi("", "u"))
}

pub fn win_clear() {
    flush_print!("{}", fansi("2J", ""))
}

pub fn del_ln() {
    flush_print!("{}", fansi("2K", ""))
}

pub fn del_to_eol() {
    flush_print!("{}", fansi("", "K"))
}

pub fn backspace() {
    flush_print!("\x08 \x08")    
}

pub fn sgr_normal() {
    flush_print!("\x1b[0m")
}

fn sgr(flags: i64) {
    if flags & BOLD == BOLD {
        flush_print!("{}", fansi(1, "m"))
    }
    if flags & HIGHLIGHT == HIGHLIGHT {
        flush_print!("{}", fansi(3, "m"))
    }
    if flags & UNDERLINE == UNDERLINE {
        flush_print!("{}", fansi(4, "m"))
    }
    if flags & STRIKETHROUGH == STRIKETHROUGH {
        flush_print!("{}", fansi(9, "m"))
    }
    if flags & FG_BLACK == FG_BLACK {
        flush_print!("{}", fansi(30, "m"))
    }
    if flags & FG_RED == FG_RED {
        flush_print!("{}", fansi(31, "m"))
    }
    if flags & FG_GREEN == FG_GREEN {
        flush_print!("{}", fansi(32, "m"))
    }
    if flags & FG_YELLOW == FG_YELLOW {
        flush_print!("{}", fansi(33, "m"))
    }
    if flags & FG_BLUE == FG_BLUE {
        flush_print!("{}", fansi(34, "m"))
    }
    if flags & FG_MAGENTA == FG_MAGENTA {
        flush_print!("{}", fansi(35, "m"))
    }
    if flags & FG_CYAN == FG_CYAN {
        flush_print!("{}", fansi(36, "m"))
    }
    if flags & FG_WHITE == FG_WHITE {
        flush_print!("{}", fansi(37, "m"))
    }
    if flags & BG_BLACK == BG_BLACK {
        flush_print!("{}", fansi(40, "m"))
    }
    if flags & BG_RED == BG_RED {
        flush_print!("{}", fansi(41, "m"))
    }
    if flags & BG_GREEN == BG_GREEN {
        flush_print!("{}", fansi(42, "m"))
    }
    if flags & BG_YELLOW == BG_YELLOW {
        flush_print!("{}", fansi(43, "m"))
    }
    if flags & BG_BLUE == BG_BLUE {
        flush_print!("{}", fansi(44, "m"))
    }
    if flags & BG_MAGENTA == BG_MAGENTA {
        flush_print!("{}", fansi(45, "m"))
    }
    if flags & BG_CYAN == BG_CYAN {
        flush_print!("{}", fansi(46, "m"))
    }
    if flags & BG_WHITE == BG_WHITE {
        flush_print!("{}", fansi(47, "m"))
    }
}


fn fansi<T, U>(x: T, y: U) -> String
    where T: Display,
          U: Display,
{
    format!("\x1b[{}{}", x, y)
}

