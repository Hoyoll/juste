use super::{io::Input, vector::Vec2};
use std::{collections::HashMap, i8, vec};

pub enum Code {
    Str(String),
    Num(i8),
    Pair(i8, String),
    None,
}

pub struct Pad {
    pub top: u32,
    pub low: u32,
    pub right: u32,
    pub left: u32,
}

pub struct Bound {
    pub pos: Vec2,
    pub dim: Vec2,
}

pub struct Style {
    pad: Pad,
    color: [u8; 4],
}

pub enum Dimension {
    Window,
    Man(u32),
}

impl Bound {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            dim: Vec2::new(width, height),
            pos: Vec2::new(0, 0),
        }
    }

    pub fn set_pos(&mut self, x: u32, y: u32) {
        self.pos.x = x;
        self.pos.y = y;
    }
}

pub enum Tag<'a> {
    None,
    Id(i32),
    Name(&'a str),
}

pub enum Genus {
    Box {
        style: Style,
        height: Dimension,
        width: Dimension,
        radius: u32,
    },
    Img {
        file_name: String,
        style: Style,
    },
    Text {
        text: String,
        size: u32,
        font_path: String,
        style: Style,
    },
}

pub struct Island<'a> {
    pub member: Vec<Element<'a>>,
    event: Option<fn(&mut Code, &mut Vec<Element<'a>>) -> Option<(i8, Code)>>,
    subscribe: Option<fn(&mut Vec<Element>, &mut HashMap<i8, Code>)>,
}

impl<'a> Island<'a> {
    pub fn new() -> Self {
        Self {
            member: vec![],
            event: None,
            subscribe: None,
        }
    }

    pub fn add_event(&mut self, fun: fn(&mut Code, &mut Vec<Element<'a>>) -> Option<(i8, Code)>) {
        self.event = Some(fun);
    }

    pub fn hear(&mut self, code: Option<Code>) -> Option<(i8, Code)> {
        if let Some(mut c) = code {
            match self.event.as_mut() {
                Some(fun) => fun(&mut c, &mut self.member),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn add_subcriber(&mut self, fun: fn(&mut Vec<Element>, &mut HashMap<i8, Code>)) {
        self.subscribe = Some(fun);
    }

    pub fn deliver(&mut self, bus: &mut HashMap<i8, Code>) {
        if let Some(fun) = self.subscribe.as_mut() {
            fun(&mut self.member, bus);
        }
    }
}

pub struct Element<'a> {
    pub genus: Genus,
    pub event: Option<fn(&mut Element, &Input) -> Option<Code>>,
    pub tag: Tag<'a>,
    pub bound: Bound,
    pub children: Option<Island<'a>>,
}

impl<'a> Element<'a> {
    pub fn new(genus_type: Genus) -> Self {
        Self {
            genus: genus_type,
            event: None,
            tag: Tag::None,
            bound: Bound::new(0, 0),
            children: None,
        }
    }

    pub fn add_event(&mut self, event: fn(&mut Element, &Input) -> Option<Code>) {
        self.event = Some(event);
    }

    pub fn add_island(&mut self, island: Island<'a>) -> &mut Self {
        self.children = Some(island);
        self
    }

    pub fn listen(&mut self, input: &Input) -> Option<Code> {
        if let Some(events) = self.event.as_ref() {
            events(self, input)
        } else {
            None
        }
    }
}
