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
    Start,
    End,
}

pub enum Prop {
    Dim(Size, Size),
    Color([u8; 4]),
    Ceil(Size, Size),
    Pad(Pad),
    Font(&'static str, u8),
    Gravity(Gravity)
}

pub enum Field {
    Color,
    Dim,
    Ceil,
    Pad,
    Font,
    Gravity
}

pub type Sheet = HashMap<Tag, HashMap<Field, Prop>>;
