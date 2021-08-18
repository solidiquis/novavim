use crate::{
    mode::{
        ctrl::Ctrl,
        Mode,
        Response
    },
    vt100,
};
use std::default::Default;

pub struct CommandLineCtrl;

impl Default for CommandLineCtrl {
    fn default() -> Self { CommandLineCtrl{} }
}

impl Ctrl for CommandLineCtrl {
    fn handle_backspace(&self) -> Response { Response::Ok }
    fn handle_return(&self) -> Response { Response::Ok }
    fn handle_up(&self) -> Response { Response::Ok }
    fn handle_down(&self) -> Response { Response::Ok }
    fn handle_right(&self) -> Response { Response::Ok }
    fn handle_left(&self) -> Response { Response::Ok }

    fn handle_esc(&self) -> Response {
        vt100::del_ln(); 
        vt100::cur_restore_pos();
        Response::ChangeMode(Mode::Normal)
    }

    fn handle_ascii(&self, ch: &str) -> Response {
        vt100::echo(ch);
        Response::Ok
    }

    fn handle_colon(&self) -> Response {
        self.handle_ascii(":")
    }
}

impl CommandLineCtrl {
    pub fn init_screen(width: usize, height: usize) {
        vt100::cur_save_pos();
        let (currow, _) = vt100::cur_pos();
        vt100::cur_down(height-currow);
        vt100::cur_right(1);
    }

}
