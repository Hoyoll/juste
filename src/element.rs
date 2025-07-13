use crate::{Gravity, Size, Style};

use super::{io::Io, vector::Vec2};
use std::{collections::HashMap, i8};

pub enum Overflow {
    Clip { active: bool },
    Leak,
}

pub struct Bound {
    pub pos: Vec2<i32>,
    pub dim: Vec2<i32>,
    pub offset: Vec2<f32>,
    pub overflow: Overflow,
    pub shadow: [i32; 4], // native representation for padding [left, right, top, low]
    pub angle: f32,
}

impl Bound {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            dim: Vec2::new(width, height),
            pos: Vec2::new(0, 0),
            offset: Vec2::new(0.0, 0.0),
            overflow: Overflow::Clip { active: false },
            shadow: [0, 0, 0, 0],
            angle: 0.0,
        }
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn inside(&self, point: &Vec2<i32>) -> bool {
        (point.x >= self.pos.x)
            && (point.x <= self.pos.x + self.dim.x)
            && (point.y >= self.pos.y)
            && (point.y <= self.pos.y + self.dim.y)
    }
}

pub trait Process {
    fn transform(&mut self, element: &mut Element);
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Tag {
    Def,
    Pass,
    Prime,
    Id(i8),
    Tup(i8, i8),
}

pub enum Genus {
    Box {
        style: Style,
        gravity: Gravity,
        size: [Size; 2],
        ceil: Option<[Size; 2]>,
        children: Vec<Element>,
    },
    Text {
        style: Style,
        text: String,
        font: &'static str,
        size: f32,
        spacing: f32,
    },
    Img {
        style: Style,
        img_path: String,
        scale: f32,
    },
}

pub struct Element {
    tag: Tag,
    genus: Genus,
    bound: Bound,
    signal_listener: Option<SignalListener>,
    io_listener: Option<IOListener>,
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

// pub struct Box {
//     color: [u8; 4],
//     pad: Pad,
//     gravity: Gravity,
//     size: [Size; 2],
//     ceil: Option<[Size; 2]>,
//     children: Vec<Element>,
// }

// pub struct Text {
//     text: String,
//     pad: Pad,
//     font: &'static str,
//     size: f32,
//     spacing: f32,
//     color: [u8; 4],
// }

// pub struct Img {
//     img_path: String,
//     pad: Pad,
//     scale: f32,
//     color: [u8; 4],
// }
// pub enum Genus {
//     Box(Tag),
//     Img(String, Tag),
//     Text(String, Tag),
// }
// impl Genus {
//     pub fn get_tag(&self) -> &Tag {
//         match self {
//             Genus::Box(tag) => tag,
//             Genus::Img(_, tag) => tag,
//             Genus::Text(_, tag) => tag,
//         }
//     }

//     pub fn replace_tag(&mut self, tag: Tag) {
//         match self {
//             Genus::Box(t) => *t = tag,
//             Genus::Img(_, t) => *t = tag,
//             Genus::Text(_, t) => *t = tag,
//         }
//     }
// }

// pub struct Element {
//     pub genus: Genus,
//     pub io_listener: Option<IOListener>,
//     pub m_listener: Option<MessageListener>,
//     pub signal_listener: Option<SignalListener>,
//     pub bound: Bound,
//     pub children: Vec<Element>,
// }

// impl Element {
//     pub fn new(genus: Genus) -> Self {
//         Self {
//             genus,
//             io_listener: None,
//             m_listener: None,
//             signal_listener: None,
//             bound: Bound::new(0, 0),
//             children: vec![],
//         }
//     }

//     pub fn add_child(&mut self, element: Element) -> &mut Self {
//         self.children.push(element);
//         self
//     }

//     pub fn io_listener(&mut self, event: IOListener) -> &mut Self {
//         self.io_listener = Some(event);
//         self
//     }

//     pub fn listen_io(&mut self, input: &Io) -> Message {
//         match self.io_listener.as_ref() {
//             Some(fun) => fun(self, input),
//             None => Message::None,
//         }
//     }

//     pub fn message_listener(&mut self, event: MessageListener) -> &mut Self {
//         self.m_listener = Some(event);
//         self
//     }

//     pub fn listen_message(&mut self, message: Message) -> Option<(i8, Message)> {
//         match self.m_listener.as_ref() {
//             Some(fun) => fun(self, message),
//             None => None,
//         }
//     }

//     pub fn signal_listener(&mut self, signal: SignalListener) -> &mut Self {
//         self.signal_listener = Some(signal);
//         self
//     }

//     pub fn listen_signal(&mut self, bus: &mut HashMap<i8, Message>) {
//         match self.signal_listener.as_ref() {
//             Some(fun) => fun(self, bus),
//             _ => (),
//         };
//     }
// }
