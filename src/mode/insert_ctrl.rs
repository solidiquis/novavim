use crate::{
    mode::{
        ctrl::Ctrl,
        Mode,
        Response
    },
    vt100
};
use std::default::Default;

pub struct InsertCtrl;

impl Default for InsertCtrl {
    fn default() -> Self { InsertCtrl{} }
}

impl Ctrl for InsertCtrl {
    fn handle_esc(&self) -> Response {
        Response::ChangeMode(Mode::Normal)    
    }

    fn handle_backspace(&self) -> Response { Response::Ok }
    fn handle_return(&self) -> Response { Response::Ok }
    fn handle_up(&self) -> Response { Response::Ok }
    fn handle_down(&self) -> Response { Response::Ok }
    fn handle_right(&self) -> Response { Response::Ok }
    fn handle_left(&self) -> Response { Response::Ok }
    fn handle_ascii(&self, ch: &str) -> Response { Response::Ok }
}
