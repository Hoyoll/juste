use std::{any::Any, char};

use num_traits::ToPrimitive;

use crate::{
    io::Io,
    style::{Color, Gravity, Size, Style, TextStyle},
};

#[derive(Debug, Clone)]
pub enum Genus {
    Input(Input),
    Box(Box),
    Img(Image),
    Text(Text),
}

#[derive(Debug, Clone)]
pub struct Input {
    pub offset: i32,
    pub cursor: Genus,
    pub state: State,
    pub stream: Stream,
    pub style: TextStyle,
}

impl Input {
    pub fn shift_cursor(&mut self, offset: i32) -> Result<(), CursorError> {
        let new_offset = self.offset + offset;

        let weight = self.stream.weight();
        if new_offset > weight {
            return Err(CursorError::BufferEnd {
                overshoot: weight - new_offset,
            });
        }

        if new_offset < 0 {
            return Err(CursorError::BufferStart {
                undershoot: new_offset,
            });
        }

        if self.offset > new_offset {
            self.stream.shift_left(self.offset - new_offset);
        } else if self.offset < new_offset {
            self.stream.shift_right(new_offset - self.offset);
        }
        self.offset = new_offset;
        Ok(())
    }

    pub fn set_cursor(&mut self, offset: i32) {
        self.offset = offset.clamp(0, self.stream.weight());
    }
}

pub struct Stream {
    pub left: Vec<Token>,
    pub right: Vec<Token>,
}

impl Stream {
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    pub fn weight(&mut self) -> i32 {
        (self.left.len() + self.right.len()) as i32
    }

    pub fn shift_left(&mut self, amount: usize) {
        for i in 0..amount {
            if let Some(token) = self.left.pop() {
                self.right.push(token);
            }
        }
    }

    pub fn shift_right(&mut self, amount: i32) {
        for i in 0..amount {
            if let Some(token) = self.right.pop() {
                self.left.push(token);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Idle,
    Active,
    Hidden,
}

pub enum CursorError {
    BufferEnd { overshoot: i32 },
    BufferStart { undershoot: i32 },
}

#[derive(Debug, Clone)]
pub enum Token {
    Space,
    Char(char),
    Indent(u8),
    Break,
}

impl Token {
    pub fn len(&self) -> i32 {
        match self {
            Token::Chars { chars, color: _ } => chars.len() as i32,
            Token::Space | Token::Break => 1,
            Token::Indent(i) => *i,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Box {
    pub style: Style,
    pub gravity: Gravity,
    pub size: [Size; 2], //[width, height]
    pub ceil: Option<[Size; 2]>,
    pub children: Vec<Element>,
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

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Font {
    File(&'static str, TTCIndex),
    Sys(&'static str, Mode),
}
pub type TTCIndex = u8;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Mode {
    Normal,
    Bold,
    Italic,
    BoldItalic,
}
