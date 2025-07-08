use std::collections::HashMap;

use super::element::Tag;

pub struct Pad {
    pub top: u32,
    pub low: u32,
    pub right: u32,
    pub left: u32,
}

pub enum Size {
    Window,
    Man(u32),
    Child,
}

pub enum Gravity {
    Horizontal,
    Vertical,
}

pub enum Style {
    Box {
        dim: (Size, Size),
        ceil: (Size, Size),
        pad: Pad,
        gravity: Gravity,
        color: [u8; 4],
    },
    Text {
        font: (&'static str, u8),
        color: [u8; 4],
        pad: Pad,
    },
    Img {
        color: [u8; 4],
        pad: Pad,
    },
}

impl Style {
    pub fn get_color(&self) -> &[u8; 4] {
        match self {
            Style::Box {
                dim: _,
                ceil: _,
                pad: _,
                gravity: _,
                color,
            } => color,
            Style::Text {
                font: _,
                color,
                pad: _,
            } => color,
            Style::Img { color, pad: _ } => color,
        }
    }
}

pub type Sheet = HashMap<Tag, Style>;
