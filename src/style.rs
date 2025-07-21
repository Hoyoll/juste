use crate::Io;

#[derive(Debug, Clone, Copy)]
pub struct Pad {
    pub top: f32,
    pub low: f32,
    pub right: f32,
    pub left: f32,
}

impl Pad {
    pub fn new() -> Self {
        Pad {
            top: 0.0,
            low: 0.0,
            right: 0.0,
            left: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Window,
    Man(f32),
    Child,
    Func(fn(&Io) -> f32),
}

#[derive(Debug, Clone, Copy)]
pub enum Gravity {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub pad: Pad,
    pub color: [u8; 4], // [r, g, b, a]
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Font {
    File(&'static str, TTCIndex),
    Sys(&'static str, Mode),
}
pub type TTCIndex = u8;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Mode {
    Normal,
    Bold,
    Italic,
}
