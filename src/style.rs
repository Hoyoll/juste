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
        font: (&'static str, f32),
        color: [u8; 4],
        pad: Pad,
        spacing: f32,
    },
    Img {
        color: [u8; 4],
        pad: Pad,
        scale: f32,
    },
}

pub type Sheet = HashMap<Tag, Style>;
