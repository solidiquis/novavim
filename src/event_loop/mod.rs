use std::{
    sync::{Arc, Mutex},
    time::Duration,
    thread::{self, sleep, JoinHandle},
};

use crate::window::Window;

pub struct EventLoop;

impl EventLoop {
   pub fn run(window: &Arc<Mutex<Window>>) -> JoinHandle<()> {
       let win = Arc::clone(window);

       thread::spawn(move || {
           loop {
               sleep(Duration::from_millis(1_000));
               Window::set_dimensions(&win, 1, 1) 
           }
       })
   } 
}
