use crate::mode::{
    ctrl::Ctrl,
    Response
};
use std::default::Default;

pub struct SelectCtrl;

impl Default for SelectCtrl {
    fn default() -> Self { SelectCtrl{} }
}

impl Ctrl for SelectCtrl {
    fn handle_backspace(&self) -> Response { Response::Ok }
    fn handle_return(&self) -> Response { Response::Ok }
    fn handle_esc(&self) -> Response { Response::Ok }
    fn handle_up(&self) -> Response { Response::Ok }
    fn handle_down(&self) -> Response { Response::Ok }
    fn handle_right(&self) -> Response { Response::Ok }
    fn handle_left(&self) -> Response { Response::Ok }
    fn handle_ascii(&self, ch: &str) -> Response { Response::Ok }
    fn handle_colon(&self) -> Response { Response::Ok }
}

impl SelectCtrl {
    pub fn init_screen(width: usize, height: usize) {}
}
