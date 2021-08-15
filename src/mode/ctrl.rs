use crate::mode::Response;

pub trait Ctrl {
    fn handle_backspace(&self) -> Response;
    fn handle_return(&self) -> Response;
    fn handle_esc(&self) -> Response;
    fn handle_up(&self) -> Response;
    fn handle_down(&self) -> Response;
    fn handle_right(&self) -> Response;
    fn handle_left(&self) -> Response;
    fn handle_ascii(&self, ch: &str) -> Response;
}
