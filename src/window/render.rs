use crate::{
    vt100::{self, constants::BOLD},
    window::Window
};

impl Window {
    pub fn init_screen(&self) {
        vt100::win_clear();
        vt100::cur_home();
        vt100::cur_down(1);

        for _ in (0..self.height-1) {
            vt100::sgrecho("~\n", 0)
        }

        vt100::cur_home();
        self.render_mode()
    }

    pub fn render_mode(&self) {
        let mode = self.mode.to_upcase_str();
        
        vt100::cur_save_pos();
        vt100::cur_set_pos(self.height, 0);
        vt100::del_ln();
        vt100::sgrecho(&mode, BOLD);
        vt100::cur_restore_pos();
    }
}
