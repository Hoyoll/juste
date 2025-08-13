use std::{collections::HashSet, mem::replace};

use crate::util::Vec2;

pub struct Io {
    pub input: Input,
    pub mouse_pos: Vec2<f32>,
    pub window_pos: Vec2<f32>,
    pub window_size: Vec2<f32>,
    pub scroll: f32,
    pub file_buff: Vec<String>,
    bucket: Option<HashSet<On>>,
}

impl Io {
    pub fn new() -> Self {
        Self {
            input: Input::None,
            mouse_pos: Vec2::new(0.0, 0.0),
            window_pos: Vec2::new(0.0, 0.0),
            window_size: Vec2::new(0.0, 0.0),
            scroll: 0.0,
            file_buff: Vec::new(),
            bucket: Some(HashSet::new()),
        }
    }

    pub fn pool(&mut self, event: On) {
        match &mut self.input {
            Input::None => {
                self.input = Input::Single(event);
            }

            Input::Single(input) => {
                let mut hash = self.bucket.take().unwrap();
                hash.insert(*input);
                hash.insert(event);
                self.input = Input::Combo(hash);
            }
            Input::Combo(hash) => {
                hash.insert(event);
            }
        }
    }
    pub fn clean(&mut self) {
        if let Input::Combo(mut hash) = replace(&mut self.input, Input::None) {
            hash.clear();
            self.bucket = Some(hash);
        }
    }
}

pub enum Input {
    Combo(HashSet<On>),
    Single(On),
    None,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum On {
    Press(From),
    Down(From),
    Release(From),
    Window(Win),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum From {
    Key(Key),
    Mouse(Mouse),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Win {
    Close,
    Resize,
    Move,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Phase {
    Start,
    Move,
    End,
    Cancel,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Point {
    Enter,
    Left,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Mouse {
    Left,
    Right,
    Middle,
    Null,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    Shift,
    Control,
    Alt,
    Meta,
    CapsLock,
    NumLock,
    ScrollLock,

    Home,
    End,
    PageUp,
    PageDown,
    Insert,
    Delete,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,

    Escape,
    Space,
    Enter,
    Backspace,
    Tab,
    Pause,
    PrintScreen,
    Menu,
    ContextMenu,
    Application,
    Power,
    Sleep,
    Wake,
    WakeUp,
    Null,
}
