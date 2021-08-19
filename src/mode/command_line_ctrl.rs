use crate::{
    commands,
    errors::Error,
    key::Key,
    mode::{
        ctrl::Ctrl,
        Mode,
        Response
    },
    vt100
};
use std::default::Default;

pub struct CommandLineCtrl{
    cmd_buffer: String
}

impl Default for CommandLineCtrl {
    fn default() -> Self {
        let cmd_buffer = "".to_string();

        CommandLineCtrl{ cmd_buffer }
    }
}

impl Ctrl for CommandLineCtrl {
    fn handle_key(&mut self, key: Key) -> Response {
        let response = match key {
            Key::Return => self.handle_return(),
            Key::ESC => self.handle_esc(),
            Key::ASCII(ch) => self.handle_ascii(ch),
            _ => Response::Ok
        };

        response
    }
}

impl CommandLineCtrl {
    pub fn init_screen(width: usize, height: usize) {
        vt100::cur_save_pos();
        let (currow, _) = vt100::cur_pos();
        vt100::cur_down(height-currow);
        vt100::cur_right(1);
    }

    // TODO: return result enum
    pub fn exec_cmd(cmd: &str) {
        match cmd {
            "q" => commands::q(),
            _ => ()
        }
    }

    fn handle_esc(&mut self) -> Response {
        self.cmd_buffer.clear();
        vt100::del_ln(); 
        vt100::cur_restore_pos();
        Response::ChangeMode(Mode::Normal)
    }

    fn handle_ascii(&mut self, ch: &str) -> Response {
        self.cmd_buffer.push_str(ch);
        vt100::echo(ch);
        Response::Ok
    }

    fn handle_return(&mut self) -> Response {
        // TODO: Error handling
        Self::exec_cmd(&self.cmd_buffer);
        self.cmd_buffer.clear();
        Response::Ok
    }
}
