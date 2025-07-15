#[derive(Debug, Clone, Copy)]
pub struct Pad {
    pub top: f32,
    pub low: f32,
    pub right: f32,
    pub left: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Window,
    Man(f32),
    Child,
}

#[derive(Debug, Clone, Copy)]
pub enum Gravity {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
pub struct Style {
    pad: Pad,
    color: [u8; 4],
}
