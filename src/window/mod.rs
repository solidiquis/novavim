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

// Maybe should have this guy own all the controllers
// as to allow each controller to have state that persists.
pub struct Window {
    mode: Mode,
    height: usize,
    width: usize,
    curcol: usize,
    currow: usize,
    normal: NormalCtrl,
    visual: VisualCtrl,
    select: SelectCtrl,
    insert: InsertCtrl,
    command_line: CommandLineCtrl
}

impl Window {
    pub fn init() -> Self {
        let (col, row) = winsize::get_winsize().unwrap();
        let height = row as usize;
        let width = col as usize;
        let curcol = 1;
        let currow = 1;
        let mode = Mode::Normal;

        let normal = NormalCtrl::default();
        let visual = VisualCtrl::default();
        let select = SelectCtrl::default();
        let insert = InsertCtrl::default();
        let command_line = CommandLineCtrl::default();

        let win = Window { 
            mode, height, width, curcol, currow,
            normal, visual, select, insert, command_line
        };

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
        let response = match self.mode {
            Mode::Normal => self.normal.handle_key(key),
            Mode::Insert => self.insert.handle_key(key),
            //Mode::Visual => self.visual.handle_key(key),
            //Mode::Select => self.select.handle_key(key)
            Mode::CommandLine => self.command_line.handle_key(key),
            _ => Response::Ok
        };

        match response {
            Response::ChangeMode(m) => self.set_mode(m),
            Response::Ok => (),
        }

        ()
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.render_mode();

        match self.mode {
            Mode::CommandLine => CommandLineCtrl::init_screen(self.width, self.height),
            _ => ()
        }
    }
}
