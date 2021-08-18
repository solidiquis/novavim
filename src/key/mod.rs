use std::str::from_utf8;

pub enum Key<'a> {
    Backspace,
    Return,
    ESC,
    Up,
    Down,
    Right,
    Left,
    Colon,
    ASCII(&'a str),
    None
}

impl<'a> Key<'a> {
    pub fn determine_key(bytes: &'a [u8; 3]) -> Self {
        match bytes[0] {
            127 => Self::Backspace,
            58  => Self::Colon,
            27  => Self::determine_special_key(bytes),
            10  => Self::Return,
            _   => Self::ASCII(Self::to_chstr(bytes))
        }
    }

    fn determine_special_key(bytes: &'a [u8; 3]) -> Self {
        match bytes[2] {
            0  => Self::ESC,
            65 => Self::Up,
            66 => Self::Down,
            67 => Self::Right,
            68 => Self::Left,
            _  => Self::None
        }
    }

    // TODO: Handle utf8 errors
    fn to_chstr(bytes: &'a [u8; 3]) -> &'a str {
        from_utf8(bytes)
            .unwrap()
            .trim_end_matches("\u{0}")
    }
}
