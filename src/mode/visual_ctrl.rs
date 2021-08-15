use crate::mode::{
    ctrl::Ctrl,
    Response
};

use std::default::Default;

pub struct VisualCtrl;

impl Default for VisualCtrl {
    fn default() -> Self { VisualCtrl{} }
}

impl Ctrl for VisualCtrl {
    fn handle_backspace(&self) -> Response { Response::Ok }
    fn handle_return(&self) -> Response { Response::Ok }
    fn handle_esc(&self) -> Response { Response::Ok }
    fn handle_up(&self) -> Response { Response::Ok }
    fn handle_down(&self) -> Response { Response::Ok }
    fn handle_right(&self) -> Response { Response::Ok }
    fn handle_left(&self) -> Response { Response::Ok }
    fn handle_ascii(&self, ch: &str) -> Response { Response::Ok }
}
