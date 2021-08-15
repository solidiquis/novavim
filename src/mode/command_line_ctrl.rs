use crate::mode::{
    ctrl::Ctrl,
    Response
};
use std::default::Default;

pub struct CommandLineCtrl;

impl Default for CommandLineCtrl {
    fn default() -> Self { CommandLineCtrl{} }
}

impl Ctrl for CommandLineCtrl {
    fn handle_backspace(&self) -> Response { Response::Ok }
    fn handle_return(&self) -> Response { Response::Ok }
    fn handle_esc(&self) -> Response { Response::Ok }
    fn handle_up(&self) -> Response { Response::Ok }
    fn handle_down(&self) -> Response { Response::Ok }
    fn handle_right(&self) -> Response { Response::Ok }
    fn handle_left(&self) -> Response { Response::Ok }
    fn handle_ascii(&self, ch: &str) -> Response { Response::Ok }
}
