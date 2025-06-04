use super::{io::Input, vector::Vec2};
use std::{collections::HashMap, vec};

pub struct Pad {
    pub top: f32,
    pub low: f32,
    pub right: f32,
    pub left: f32,
}

pub struct Bound {
    pub pos: Vec2,
    pub dim: Vec2,
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub struct Properties {
    pad: Pad,
    color: Color,
}

impl Bound {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            pos: Vec2::new(width, height),
            dim: Vec2::new(0.0, 0.0),
        }
    }

    pub fn get_dim(self) -> Vec2 {
        self.dim
    }

    pub fn get_pos(self) -> Vec2 {
        self.pos
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }
}

pub struct Event {
    events: HashMap<&'static str, fn(&mut Genus)>,
    pub single: HashMap<&'static str, Input>,
    pub combo: HashMap<&'static str, Input>,
}

impl Event {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            single: HashMap::new(),
            combo: HashMap::new(),
        }
    }

    pub fn on(&mut self, whistle: &'static str, fun: fn(&mut Genus)) {
        self.events.insert(whistle, fun);
    }

    pub fn when(&mut self, input: Input, whistle: &'static str) {
        match input {
            Input::Combo(_) => self.combo.insert(whistle, input),
            Input::Single(_) => self.single.insert(whistle, input),
        };
    }

    pub fn call(&mut self, whistle: &'static str) -> Option<&for<'a> fn(&'a mut Genus)> {
        self.events.get(whistle)
    }
}

pub enum Genus {
    Box {
        dim: Vec2,
        prop: Properties,
    },
    Img {
        file_name: String,
        prop: Properties,
    },
    Text {
        text: String,
        font_path: &'static str,
        prop: Properties,
    },
}

pub struct Element {
    pub genus: Genus,
    pub event: Option<Event>,
    pub composer: Option<Bound>,
    pub childs: Option<Vec<usize>>,
}

impl Element {
    pub fn new(genus_type: Genus) -> Self {
        Self {
            genus: genus_type,
            event: None,
            composer: None,
            childs: None,
        }
    }

    pub fn add_event(&mut self, event: Event) {
        self.event = Some(event);
    }

    pub fn add_child(&mut self, children: usize) {
        match self.childs.as_mut() {
            Some(childs) => {
                childs.push(children);
            }
            None => self.childs = Some(vec![children]),
        }
    }
}
