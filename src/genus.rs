use std::char;

use crate::{
    element::Element,
    io::Io,
    style::{Color, Gravity, Size, Style, TextStyle},
    util::{GapBuf, Vec2},
};

#[derive(Debug, Clone)]
pub enum Genus {
    Input(Input),
    Box(Box),
    Cult(Box),
    Img(Image),
    Text(Text),
}

#[derive(Debug, Clone)]
pub struct Input {
    pub cursor: Cursor,
    pub state: State,
    pub stream: GapBuf<Token>,
    pub style: TextStyle,
    pub size: Vec2<Size>,
    pub token_size: Vec2<f32>,
}

#[derive(Debug, Clone)]
pub struct Cursor {
    pub size: Vec2<Size>, //[width, height]
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Space,
    Char(char),
}

#[derive(Debug, Clone)]
pub enum State {
    Idle,
    Active,
    Hidden,
}

#[derive(Debug, Clone)]
pub struct Box {
    pub style: Style,
    pub gravity: Gravity,
    pub size: Vec2<Size>,         //[width, height]
    pub ceil: Option<Vec2<Size>>, //[width, height]
    pub children: Option<Child>,
}

#[derive(Debug, Clone)]
pub enum Child {
    Gap(GapBuf<Element>),
    Vec(Vec<Element>),
}

impl Child {
    pub fn iter_mut<F>(&mut self, fun: F)
    where
        F: FnMut(&mut Element),
    {
        match self {
            Child::Gap(gap) => {
                gap.iter_mut(fun);
            }
            Child::Vec(vec) => {
                vec.iter_mut().for_each(fun);
            }
        }
    }
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
    pub text: String,
    pub style: TextStyle,
}
