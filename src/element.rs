use super::{io::Io, vector::Vec2};
use crate::{Font, Gravity, Size, Style};
use std::{collections::HashMap, i8};

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

#[derive(Debug, Clone, Copy)]
pub struct Bound {
    pub pos: Vec2<f32>,
    pub dim: Vec2<f32>,
    pub offset: Vec2<f32>,
    pub overflow: Overflow,
    pub shadow: [f32; 4], // native representation for padding [left, right, top, low]
    pub angle: f32,
}
impl Bound {
    pub fn new() -> Self {
        Self {
            dim: Vec2::new(0.0, 0.0),
            pos: Vec2::new(0.0, 0.0),
            offset: Vec2::new(0.0, 0.0),
            overflow: Overflow::Clip { active: false },
            shadow: [0.0, 0.0, 0.0, 0.0],
            angle: 0.0,
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn set_dim(&mut self, x: f32, y: f32) {
        self.dim.x = x;
        self.dim.y = y;
    }

    pub fn inside(&self, point: &Vec2<f32>) -> bool {
        (point.x >= self.pos.x)
            && (point.x <= self.pos.x + self.dim.x)
            && (point.y >= self.pos.y)
            && (point.y <= self.pos.y + self.dim.y)
    }
}

pub trait Process {
    fn message(&mut self, _message: Message);
    fn transform(&mut self, _element: &mut Element);
    fn destroy(&mut self);
}

pub enum Message {
    Num(i8),
    Tup(i8, i8),
    Str(String),
    Pair(i8, String),
    Proc(Box<dyn Process>),
}

pub type SignalBus = HashMap<Tag, Message>;
pub type IOListener = fn(&mut Element, &Io) -> Option<(Tag, Message)>;
pub type SignalListener = fn(&mut Element, &mut SignalBus);

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub enum Tag {
    Def,
    None,
    Prime,
    Id(i8),
    Tup(i8, i8),
}

#[derive(Debug, Clone)]
pub enum Genus {
    Box {
        style: Style,
        gravity: Gravity,
        size: [Size; 2], //[width, height]
        ceil: Option<[Size; 2]>,
        children: Vec<Element>,
    },
    Text {
        style: Style,
        text: String,
        font: Font,
        size: f32,
        spacing: f32,
    },
    Img {
        style: Style,
        img_path: String,
        scale: f32,
    },
}
#[derive(Debug, Clone)]
pub struct Element {
    pub tag: Tag,
    pub genus: Genus,
    pub bound: Bound,
    pub signal_listener: Option<SignalListener>,
    pub io_listener: Option<IOListener>,
}

impl Element {
    pub fn listen_io(&mut self, io: &Io) -> Option<(Tag, Message)> {
        match self.io_listener.as_ref() {
            Some(fun) => fun(self, io),
            None => None,
        }
    }

    pub fn listen_signal(&mut self, bus: &mut SignalBus) {
        match self.signal_listener.as_ref() {
            Some(fun) => fun(self, bus),
            None => (),
        }
    }

    pub fn set_io_listener(&mut self, fun: IOListener) {
        self.io_listener = Some(fun);
    }
    pub fn set_signal_listener(&mut self, fun: SignalListener) {
        self.signal_listener = Some(fun);
    }
}
