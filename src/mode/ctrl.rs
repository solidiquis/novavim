use crate::{
    key::Key,
    mode::Response
};

pub trait Ctrl {
    fn handle_key(&mut self, key: Key) -> Response;
}
