use std::char;

use crate::{
    element::Element,
    io::Io,
    style::{ColorId, Gravity, Size, Style, TextStyle},
    util::{GapBuf, Vec2},
};

#[derive(Debug, Clone)]
pub enum Genus {
    Input(Input),
    Frame(Frame),
    Cult(Frame),
    Float(Frame),
    Img(Image),
    Text(Text),
}

#[derive(Debug, Clone)]
pub struct Input {
    pub cursor: Cursor,
    pub state: State,
    pub offset: Vec2<f32>,
    pub stream: GapBuf<Token>,
    pub style: TextStyle,
    pub token_size: Vec2<f32>,
}

#[derive(Debug, Clone)]
pub struct Cursor {
    pub width: f32,
    pub color: ColorId,
}
// #[derive(Debug, Clone, Copy)]
// pub enum Unit<T: Clone> {
//     Man(T),
//     Batch(T),
// }

#[derive(Debug, Clone)]
pub enum Token {
    Space,
    Break,
    Char(GapBuf<char>),
}

#[derive(Debug, Clone)]
pub enum State {
    Idle,
    Active,
    Hidden,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub style: Style,
    pub gravity: Gravity,
    pub overflow: Overflow,
    pub child_offset: Vec2<f32>,
    pub size: Vec2<Size>,         //[width, height]
    pub ceil: Option<Vec2<Size>>, //[width, height]
    pub children: Option<Child>,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            style: Style::new(),
            gravity: Gravity::Horizontal,
            overflow: Overflow::Leak,
            child_offset: Vec2::new(0.0, 0.0),
            size: Vec2::new(Size::Window, Size::Window),
            ceil: None,
            children: None,
        }
    }

    pub fn with_children(&mut self, children: Child) -> &Self {
        self.children = Some(children);
        self
    }
}

#[derive(Debug, Clone)]
pub enum Child {
    Gap(GapBuf<Element>),
    Vec(Vec<Element>),
}

#[derive(Debug, Clone, Copy)]
pub enum Overflow {
    Clip { active: bool },
    Leak,
}

impl Overflow {
    pub fn make_clip(&mut self) {
        match self {
            Overflow::Clip { active } => {
                *active = true;
            }
            _ => (),
        }
    }

    pub fn need_clip(&mut self) -> bool {
        if let Overflow::Clip { active: true } = self {
            true
        } else {
            false
        }
    }
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

    pub fn iter<F>(&self, fun: F)
    where
        F: FnMut(&Element),
    {
        match self {
            Child::Gap(gap) => {
                gap.iter(fun);
            }
            Child::Vec(vec) => {
                vec.iter().for_each(fun);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pub style: Style,
    pub img_path: Src,
    pub fallback: fn(&Io) -> Element,
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
