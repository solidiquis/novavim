use crate::{
    key::Key,
    mode::{
        ctrl::Ctrl,
        Mode,
        Response
    }
};
use std::default::Default;

pub struct NormalCtrl;

impl Default for NormalCtrl {
    fn default() -> Self { NormalCtrl{} }
}

impl Ctrl for NormalCtrl {
    fn handle_key(&mut self, key: Key) -> Response {
        let response = match key {
            Key::ASCII(ch) => self.handle_ascii(ch),
            Key::ESC => self.handle_esc(),
            Key::Colon => self.handle_colon(),
            _ => Response::Ok
        };

        response
    }
}

impl NormalCtrl {
    fn handle_esc(&mut self) -> Response {
        Response::Ok
    }

    fn handle_ascii(&self, ch: &str) -> Response {
        match ch {
            "i" => Response::ChangeMode(Mode::Insert),
            _ => Response::Ok
        }
    }

    fn handle_colon(&self) -> Response {
        Response::ChangeMode(Mode::CommandLine)
    }
}
