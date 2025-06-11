use super::{io::Input, vector::Vec2};
use std::vec;

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
    Man(f32),
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

pub enum Tag {
    None,
    Id(i32),
    Name(String),
}

pub enum Genus {
    Box {
        dim: Vec2,
        style: Style,
        height: Dimension,
        width: Dimension,
    },
    Img {
        file_name: String,
        style: Style,
    },
    Text {
        text: String,
        font_path: &'static str,
        style: Style,
    },
}

pub struct Element {
    pub genus: Genus,
    pub event: Option<fn(&mut Element, Option<&Input>, &Vec<Element>)>,
    pub tag: Tag,
    pub bound: Option<Bound>,
    pub children: Option<Vec<Element>>,
}

impl Element {
    pub fn new(genus_type: Genus) -> Self {
        Self {
            genus: genus_type,
            event: None,
            tag: Tag::None,
            bound: None,
            children: None,
        }
    }

    pub fn add_event(&mut self, event: fn(&mut Element, Option<&Input>, &Vec<Element>)) {
        self.event = Some(event);
    }

    pub fn add_child(&mut self, children: Element) -> &mut Self {
        match self.children.as_mut() {
            Some(childs) => {
                childs.push(children);
            }
            None => self.children = Some(vec![children]),
        }
        self
    }

    pub fn listen(&mut self, input: Option<&Input>, cluster: &Vec<Element>) {
        if let Some(events) = self.event.as_ref() {
            events(self, input, cluster)
        }
    }
}
