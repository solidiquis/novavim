mod winsize;
use std::sync::{Arc, Mutex};

pub struct Window {
    height: usize,
    width: usize,
    curcol: usize,
    currow: usize
}

impl Window {
    pub fn init() -> Self {
        let (col, row) = winsize::get_winsize().unwrap();
        let height = row as usize;
        let width = col as usize;
        let curcol = 1;
        let currow = 1;

        Window { height, width, curcol, currow }
    }

    pub fn set_dimensions(window: &Arc<Mutex<Self>>, width: usize, height: usize) {
        let win = Arc::clone(window);
        let mut win = win.lock().unwrap();
        win.width = width;
        win.height = height
    }

}
