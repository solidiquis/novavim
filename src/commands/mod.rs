//use crate::errors::Error;

use crate::vt100;

pub fn q() {
    vt100::win_clear();
    vt100::cur_home();
    std::process::exit(1)
}
