mod commands;
mod errors;
mod event_loop;
mod key;
mod mode;
mod vt100;
mod window;

use event_loop::EventLoop;
use libc::c_int;
use std::{
    error::Error,
    io::stdin,
    os::unix::io::AsRawFd,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle}
};
use signal_hook::{
    consts::signal::{SIGWINCH, SIGINT},
    iterator::{
        SignalsInfo,
        exfiltrator::origin::WithOrigin
    }
};
use termios::{tcsetattr, Termios};
use window::Window;

const TCSANOW: c_int = 0;
const VMIN: usize = 6;

fn main() -> Result<(), Box<dyn Error>> {
    let kernal_sigs = vec![SIGWINCH, SIGINT];
    let mut sig_info = SignalsInfo::<WithOrigin>::new(&kernal_sigs)?;

    let char_device = stdin().as_raw_fd();
    let mut termios = Termios::from_fd(char_device)?;

    termios.c_lflag ^= termios::ECHO | termios::ICANON;
    termios.c_cc[VMIN] = 1;

    tcsetattr(char_device, TCSANOW, &termios).unwrap();

    let win = Arc::new(Mutex::new(Window::init()));
    let _el = exec_loop(&win);

    for info in &mut sig_info {
        match info.signal {
            SIGWINCH => Window::set_dimensions_with_lock(&win, 1, 1),
            SIGINT | _ => (),
        }
    }

    Ok(())
}

fn exec_loop(window: &Arc<Mutex<Window>>) -> JoinHandle<()> {
    let win = Arc::clone(window);

    thread::spawn(move || {
        EventLoop::run(&win)
    })        
}

