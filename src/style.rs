use std::collections::HashMap;

use super::element::Tag;

pub struct Pad {
    pub top: i32,
    pub low: i32,
    pub right: i32,
    pub left: i32,
}

pub enum Size {
    Window,
    Man(i32),
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
