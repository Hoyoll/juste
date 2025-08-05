use crate::io::Io;

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
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub pad: Pad,
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct TextStyle {
    pub font: Font,
    pub spacing: f32,
    pub style: Style,
    pub fallback: Option<Fallback>,
}

#[derive(Debug, Clone, Copy)]
pub enum Fallback {
    Static(Font),
    List(Vec<Font>),
    Dyn(fn(&Io) -> Font),
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Font {
    File {
        path: &'static str,
        size: f32,
        ttc: TTCIndex,
    },
    Sys {
        name: &'static str,
        size: f32,
        mode: Mode,
    },
}

impl Font {
    pub fn get_size(&self) -> f32 {
        match self {
            Font::File { path: _, size, .. } => *size,
            Font::Sys { name: _, size, .. } => *size,
        }
    }
}

pub type TTCIndex = u8;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Mode {
    Normal,
    Bold,
    Italic,
    BoldItalic,
}
