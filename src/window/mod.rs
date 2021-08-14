mod winsize;

use std::sync::{Arc, Mutex};

pub struct Window {
    height: usize,
    width: usize,
}

impl Window {
    pub fn init() -> Self {
        let (col, row) = winsize::get_winsize().unwrap();
        let height = row as usize;
        let width = col as usize;

        Window { height, width }
    }

    pub fn set_dimensions(window: &Arc<Mutex<Self>>, width: usize, height: usize) {
        let win = Arc::clone(window);
        let mut win = win.lock().unwrap();
        win.width = width;
        win.height = height
    }
}
