use crate::key::Key;
use crate::window::Window;

use std::{
    io::{self, Read},
    process::Command,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

pub struct EventLoop;

impl EventLoop {
    pub fn run(window: &Arc<Mutex<Window>>) {
        let mut stdin = io::stdin();
        let mut buffer = [0; 3];
        let mut key;

        loop {
            stdin.read(&mut buffer).unwrap();
            key = Key::determine_key(&buffer);

            let mut win = window.lock().unwrap();
            win.handle_key(key);

            Self::reinit_keybuff(&mut buffer)
        }
    } 

    fn reinit_keybuff(bytes: &mut [u8; 3]) {
        *bytes = [0; 3]
    }
}
