use crate::{
    element::Element,
    io::Io,
    style::{Gravity, Size, Style},
};

#[derive(Debug, Clone)]
pub enum Genus {
    Input(Input),
    Box(Box),
    Img(Image),
    Text(Text),
}

#[derive(Debug, Clone)]
pub struct Input {
    pub offset: usize,
    pub cursor: Box,
    pub state: State,
    pub stream: Vec<Token>,
    pub font: Font,
    pub size: f32,
    pub spacing: f32,
    pub style: Style,
}

#[derive(Debug, Clone)]
pub enum State {
    Idle,
    Active,
}

#[derive(Debug, Clone)]
pub enum Token {
    Chars(Vec<char>),
    Space,
    Break,
}

#[derive(Debug, Clone)]
pub struct Box {
    pub style: Style,
    pub gravity: Gravity,
    pub size: [Size; 2], //[width, height]
    pub ceil: Option<[Size; 2]>,
    pub children: Vec<Element>,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub style: Style,
    pub img_path: Src,
    pub fallback: Option<fn(&Io) -> Element>,
    pub scale: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Src {
    Sys(String),
    Url(String),
}

#[derive(Debug, Clone)]
pub struct Text {
    pub style: Style,
    pub text: String,
    pub font: Font,
    pub size: f32,
    pub spacing: f32,
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
    BoldItalic,
}
