use crate::{
    key::Key,
    mode::{
        Mode,
        Response,
        ctrl::Ctrl,
        normal_ctrl::NormalCtrl,
        visual_ctrl::VisualCtrl,
        insert_ctrl::InsertCtrl,
        select_ctrl::SelectCtrl,
        command_line_ctrl::CommandLineCtrl
    },
    vt100
};
mod render;
mod winsize;

use std::sync::{Arc, Mutex};

pub struct Window {
    mode: Mode,
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
        let mode = Mode::Normal;
        let win = Window { mode, height, width, curcol, currow };
        win.init_screen();

        win
    }

    pub fn set_dimensions_with_lock(window: &Arc<Mutex<Self>>, width: usize, height: usize) {
        let win = Arc::clone(window);
        let mut win = win.lock().unwrap();
        win.width = width;
        win.height = height
    }

    pub fn handle_key(&mut self, key: Key) {
        let ctrl: Box<dyn Ctrl> = match self.mode {
            Mode::Normal => Box::new(NormalCtrl::default()),
            Mode::Insert => Box::new(InsertCtrl::default()),
            Mode::Visual => Box::new(VisualCtrl::default()),
            Mode::CommandLine => Box::new(CommandLineCtrl::default()),
            Mode::Select => Box::new(SelectCtrl::default())
        };

        let response = match key {
            Key::Backspace => ctrl.handle_backspace(),
            Key::Return => ctrl.handle_return(),
            Key::ESC => ctrl.handle_esc(),
            Key::Up => ctrl.handle_up(),
            Key::Down => ctrl.handle_down(),
            Key::Right => ctrl.handle_right(),
            Key::Left => ctrl.handle_left(),
            Key::ASCII(s) => ctrl.handle_ascii(s),
            Key::None => Response::Ok,
        };

        match response {
            Response::ChangeMode(m) => self.set_mode(m),
            Response::Ok => (),
        }

        ()
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.render_mode()
    }
}
