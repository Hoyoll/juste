use crate::{genus::Genus, io::Io, util::Vec2};
use std::{collections::HashMap, fmt, i8};

#[derive(Debug, Clone, Copy)]
pub struct Bound {
    pub pos: Vec2<f32>,
    pub dim: Vec2<f32>,
    pub offset: Vec2<f32>,
    pub shadow: [f32; 4], // native representation for padding [left, right, top, low]
    pub angle: Option<f32>,
}
impl Bound {
    pub fn new() -> Self {
        Self {
            dim: Vec2::new(0.0, 0.0),
            pos: Vec2::new(0.0, 0.0),
            offset: Vec2::new(0.0, 0.0),
            shadow: [0.0, 0.0, 0.0, 0.0],
            angle: None,
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

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub enum Tag {
    Def,
    None,
    Prime,
    Id(i8),
    Tup(i8, i8),
}

pub enum Message {
    Num(i8),
    Tup(i8, i8),
    Str(String),
    Pair(i8, String),
    Proc(Box<dyn Process>),
}

pub trait Process {
    fn message(&mut self, message: Message);
    fn transform(&mut self, element: &mut Element);
    fn destroy(&mut self);
}

pub trait Transform: TransformClone {
    fn io_listener(&mut self, element: &mut Element, io: &Io) -> Option<(Tag, Message)>;
    fn signal_listener(&mut self, element: &mut Element, signal: &mut SignalBus);
}

pub trait TransformClone {
    fn clone_box(&self) -> Box<dyn Transform>;
}

impl<T> TransformClone for T
where
    T: 'static + Transform + Clone,
{
    fn clone_box(&self) -> Box<dyn Transform> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Transform> {
    fn clone(&self) -> Box<dyn Transform> {
        self.clone_box()
    }
}

impl fmt::Debug for Listener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Listener::Pure {
                io_listener,
                signal_listener,
            } => f
                .debug_struct("Pure")
                .field("io_listener", io_listener)
                .field("signal_listener", signal_listener)
                .finish(),
            Listener::Trans(_) => f.debug_tuple("Trans").field(&"<Transform>").finish(),
        }
    }
}

pub type SignalBus = HashMap<Tag, Message>;
pub type IOListener = fn(&mut Element, &Io) -> Option<(Tag, Message)>;
pub type SignalListener = fn(&mut Element, &mut SignalBus);

#[derive(Clone)]
pub enum Listener {
    Pure {
        io_listener: Option<IOListener>,
        signal_listener: Option<SignalListener>,
    },
    Trans(Box<dyn Transform>),
}

impl Listener {
    pub fn listen_io(&mut self, element: &mut Element, io: &Io) -> Option<(Tag, Message)> {
        match self {
            Listener::Pure {
                io_listener,
                signal_listener: _,
            } => match io_listener {
                Some(fun) => fun(element, io),
                None => None,
            },
            Listener::Trans(tran) => tran.io_listener(element, io),
        }
    }

    pub fn listen_bus(&mut self, element: &mut Element, bus: &mut SignalBus) {
        match self {
            Listener::Pure {
                io_listener: _,
                signal_listener,
            } => match signal_listener {
                Some(fun) => fun(element, bus),
                None => (),
            },
            Listener::Trans(tran) => tran.signal_listener(element, bus),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    pub tag: Tag,
    pub genus: Genus,
    pub bound: Bound,
    pub listener: Option<Listener>,
}

impl Element {
    pub fn listen_io(&mut self, io: &Io) -> Option<(Tag, Message)> {
        if let Some(mut list) = self.listener.take() {
            let res = list.listen_io(self, io);
            self.listener = Some(list);
            res
        } else {
            None
        }
    }
    pub fn listen_signal(&mut self, bus: &mut SignalBus) {
        if let Some(mut list) = self.listener.take() {
            list.listen_bus(self, bus);
            self.listener = Some(list);
        }
    }
}
