pub mod normal_ctrl;
pub mod insert_ctrl;
pub mod visual_ctrl;
pub mod command_line_ctrl;
pub mod select_ctrl;
pub mod ctrl;

pub enum Mode {
    Normal,
    Insert,
    Visual,
    CommandLine,
    Select
}

pub enum Response {
    Ok,
    ChangeMode(Mode)
}

impl Mode {
    pub fn to_upcase_str(&self) -> String {
        match self {
            Self::Normal => "".to_string(),
            Self::Insert => "-- INSERT --".to_string(),
            Self::Visual => "-- VISUAL --".to_string(),
            Self::Select => "/".to_string(),
            Self::CommandLine => ":".to_string()
        }
    }
}
