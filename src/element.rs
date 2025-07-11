use super::{io::Io, vector::Vec2};
use std::{collections::HashMap, i8, vec};

pub enum Overflow {
    Clip { active: bool },
    Leak,
}

pub struct Bound {
    pub pos: Vec2<u32>,
    pub dim: Vec2<u32>,
    pub offset: Vec2<f32>,
    pub overflow: Overflow,
    pub shadow: [f32; 4], // native representation for padding [left, right, top, low]
    pub angle: f32,
}

impl Bound {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            dim: Vec2::new(width, height),
            pos: Vec2::new(0, 0),
            offset: Vec2::new(0.0, 0.0),
            overflow: Overflow::Clip { active: false },
            shadow: [0.0, 0.0, 0.0, 0.0],
            angle: 0.0,
        }
    }

    pub fn set_pos(&mut self, x: u32, y: u32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn inside(&self, point: &Vec2<u32>) -> bool {
        (point.x >= self.pos.x)
            && (point.x <= self.pos.x + self.dim.x)
            && (point.y >= self.pos.y)
            && (point.y <= self.pos.y + self.dim.y)
    }
}

pub enum Message {
    Num(i8),
    Tup(i8, i8),
    None,
}

pub type IOListener = fn(&mut Element, &Io) -> Message;
pub type MessageListener = fn(&mut Element, Message) -> Option<(i8, Message)>;
pub type SignalListener = fn(&mut Element, &mut HashMap<i8, Message>);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Tag {
    None,
    Id(i8),
    Tup(i8, i8),
}

pub enum Genus {
    Box(Tag),
    Img(String, Tag),
    Text(String, Tag),
}
impl Genus {
    pub fn get_tag(&self) -> &Tag {
        match self {
            Genus::Box(tag) => tag,
            Genus::Img(_, tag) => tag,
            Genus::Text(_, tag) => tag,
        }
    }

    pub fn replace_tag(&mut self, tag: Tag) {
        match self {
            Genus::Box(t) => *t = tag,
            Genus::Img(_, t) => *t = tag,
            Genus::Text(_, t) => *t = tag,
        }
    }
}

pub struct Element {
    pub genus: Genus,
    pub io_listener: Option<IOListener>,
    pub m_listener: Option<MessageListener>,
    pub signal_listener: Option<SignalListener>,
    pub bound: Bound,
    pub children: Vec<Element>,
}

impl Element {
    pub fn new(genus: Genus) -> Self {
        Self {
            genus,
            io_listener: None,
            m_listener: None,
            signal_listener: None,
            bound: Bound::new(0, 0),
            children: vec![],
        }
    }

    pub fn io_listener(&mut self, event: IOListener) {
        self.io_listener = Some(event);
    }

    pub fn listen_io(&mut self, input: &Io) -> Message {
        match self.io_listener.as_ref() {
            Some(fun) => fun(self, input),
            None => Message::None,
        }
    }

    pub fn message_listener(&mut self, event: MessageListener) {
        self.m_listener = Some(event);
    }

    pub fn listen_message(&mut self, message: Message) -> Option<(i8, Message)> {
        match self.m_listener.as_ref() {
            Some(fun) => fun(self, message),
            None => None,
        }
    }

    pub fn signal_listener(&mut self, signal: SignalListener) {
        self.signal_listener = Some(signal);
    }

    pub fn listen_signal(&mut self, bus: &mut HashMap<i8, Message>) {
        match self.signal_listener.as_ref() {
            Some(fun) => fun(self, bus),
            _ => (),
        };
    }
}
