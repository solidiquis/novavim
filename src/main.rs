mod event_loop;
use event_loop::EventLoop;
mod key;
mod mode;
mod vt100;
mod window;
use window::Window;

use std::{
    io::Error,
    process::Command,
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

fn main() -> Result<(), Error> {
    //let os_signals = vec![SIGWINCH, SIGINT];
    let os_signals = vec![SIGWINCH];
    let mut signal_info = SignalsInfo::<WithOrigin>::new(&os_signals)?;

    unbuffer_stdin();
    unecho_stdin();

    let win = Arc::new(Mutex::new(Window::init()));
    let _el = exec_loop(&win);

    for info in &mut signal_info {
        match info.signal {
            SIGWINCH => Window::set_dimensions_with_lock(&win, 1, 1),
            //SIGINT => handle,
            _ => ()
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

fn unbuffer_stdin() {
    Command::new("stty")
        .arg("-f")
        .arg("/dev/tty")
        .arg("cbreak")
        .arg("min")
        .arg("1")
        .output()
        .expect("Failed to unbuffer stdin.");
    ()
}

fn unecho_stdin() {
    Command::new("stty")
        .arg("-f")
        .arg("/dev/tty")
        .arg("-echo")
        .output()
        .expect("Failed to unecho stdin.");
    ()
}
