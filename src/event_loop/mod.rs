use std::{
    io::{self, Read},
    process::Command,
    sync::{Arc, Mutex},
    thread::{self, sleep, JoinHandle},
};

use crate::key::Key;
use crate::window::Window;

pub struct EventLoop;

impl EventLoop {
    pub fn run(window: &Arc<Mutex<Window>>) -> JoinHandle<()> {
        Self::unbuffer_stdin();
        Self::unecho_stdin();

        let win = Arc::clone(window);

        thread::spawn(move || {
            let mut stdin = io::stdin();
            let mut buffer = [0; 3];
            let mut key;

            loop {
                stdin.read(&mut buffer).unwrap();
                key = Key::determine_key(&buffer);
                // handle key
                Self::reinit_keybuff(&mut buffer)
            }
        })
    } 

    fn reinit_keybuff(bytes: &mut [u8; 3]) {
        *bytes = [0; 3]
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
}
