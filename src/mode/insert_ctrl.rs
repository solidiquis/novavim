use crate::{
    key::Key,
    mode::{
        ctrl::Ctrl,
        Mode,
        Response
    }
};
use std::default::Default;

pub struct InsertCtrl;

impl Default for InsertCtrl {
    fn default() -> Self { InsertCtrl{} }
}

impl Ctrl for InsertCtrl {
    fn handle_key(&mut self, key: Key) -> Response {
        let response = match key {
            Key::ESC => self.handle_esc(),
            _ => Response::Ok
        };

        response
    }
}

impl InsertCtrl {
    fn handle_esc(&mut self) -> Response {
        Response::ChangeMode(Mode::Normal)    
    }

}
