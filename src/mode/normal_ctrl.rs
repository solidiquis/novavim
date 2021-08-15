use crate::mode::{
    ctrl::Ctrl,
    Mode,
    Response
};
use std::default::Default;

pub struct NormalCtrl;

impl Default for NormalCtrl {
    fn default() -> Self { NormalCtrl{} }
}

impl Ctrl for NormalCtrl {
    fn handle_ascii(&self, ch: &str) -> Response {
        match ch {
            "i" => Response::ChangeMode(Mode::Insert),
            _ => Response::Ok
        }
    }

    fn handle_backspace(&self) -> Response { Response::Ok }
    fn handle_return(&self) -> Response { Response::Ok }
    fn handle_esc(&self) -> Response { Response::Ok }
    fn handle_up(&self) -> Response { Response::Ok }
    fn handle_down(&self) -> Response { Response::Ok }
    fn handle_right(&self) -> Response { Response::Ok }
    fn handle_left(&self) -> Response { Response::Ok }
}
