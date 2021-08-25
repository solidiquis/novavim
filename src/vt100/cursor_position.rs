use crate::vt100::error;
use regex::Regex;
use std::{
    error::Error,
    io::{self, stdin, Read, Write},
    os::unix::io::AsRawFd,
    str
};
use termios::{self, Termios};

pub fn query_cursor_position() -> Result<(usize, usize), Box<dyn Error>> {
    // Safety check.. but perhaps a bit too paranoid?
    let char_device = stdin().as_raw_fd();
    let termios = Termios::from_fd(char_device)?;
    
    if termios.c_lflag & (termios::ICANON | termios::ECHO) == (termios::ICANON | termios::ECHO) {
        return Err(Box::new(error::Error::RawModeRequired))
    }

    // query the cursor position.
    let mut stdout = io::stdout();
    stdout.write_all(b"\x1b[6n");
    stdout.flush()?;

    // Hack alert: This is me assuming that nobody has
    // unreasonably sized windows which would cause the
    // response from the device to be inordinately large
    // e.g. "ESC[10000;10000R. If and when that becomes an
    // issue, we will make the syscall necessary to get the
    // window size each and every time the cursor position
    // is queried; but until then, we hackin our blues away.
    let mut buffer = [0; 10];

    // Read device report into buffer.
    stdin().read(&mut buffer)?;

    let result = str::from_utf8(&buffer)?;

    let cur_pos: Vec<&str> = Regex::new(r"\d+;\d+")
        .unwrap()
        .find(&result)
        .unwrap()
        .as_str()
        .split(';')
        .collect();

    let pos: Vec<usize> = cur_pos
        .into_iter()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    if let [row, col] = pos[..] {
        Ok((row, col))
    } else {
        Err(Box::new(error::Error::CurPosErr))
    }
}
