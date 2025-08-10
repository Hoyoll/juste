use std::{collections::HashMap, hash::Hash};

use crate::{element::Listeners, io::Io};

#[derive(Debug, Clone, Copy)]
pub struct Pad {
    pub top: f32,
    pub low: f32,
    pub right: f32,
    pub left: f32,
}

impl Pad {
    pub fn new() -> Self {
        Pad {
            top: 0.0,
            low: 0.0,
            right: 0.0,
            left: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Window,
    Man(f32),
    Child,
    Func(fn(&Io) -> f32),
}

#[derive(Debug, Clone, Copy)]
pub enum Gravity {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Origin<T: Eq + Hash, V> {
    Id(T),
    Raw(V),
}

#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub pad: PadId,
    pub color: ColorId,
}

impl Style {
    pub fn new() -> Self {
        Self {
            pad: DEFAULT,
            color: DEFAULT,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TextStyle {
    pub font: FontId,
    //pub spacing: f32,
    pub style: Style,
    //pub fallback: fn(&Io) -> Origin<FontId, Font>,
}

impl TextStyle {
    pub fn new() -> Self {
        Self {
            font: DEFAULT,
            style: Style::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Font {
    File {
        path: &'static str,
        size: usize,
        ttc: TTCIndex,
    },
    Sys {
        name: &'static str,
        size: usize,
        mode: Mode,
    },
}

impl Font {
    pub fn get_size(&self) -> usize {
        match self {
            Font::File { path: _, size, .. } => *size,
            Font::Sys { name: _, size, .. } => *size,
        }
    }
}

pub type TTCIndex = usize;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Mode {
    Normal,
    Bold,
    Italic,
    BoldItalic,
}

pub const DEFAULT: i8 = -1;

pub type FontId = i8;
pub type ColorId = i8;
pub type PadId = i8;

pub type FontSheet = HashMap<FontId, Font>;
pub type ColorSheet = HashMap<ColorId, Color>;
pub type PadSheet = HashMap<PadId, Pad>;

pub struct Sheet {
    pub fonts: FontSheet,
    pub colors: ColorSheet,
    pub pads: PadSheet,
    pub listener: Listeners,
}

impl Sheet {
    pub fn new() -> Self {
        let mut fonts = HashMap::new();
        fonts.insert(
            DEFAULT,
            Font::Sys {
                name: "Arial",
                size: 14,
                mode: Mode::Normal,
            },
        );
        let mut colors = HashMap::new();
        colors.insert(DEFAULT, Color::new());
        let mut pads = HashMap::new();
        pads.insert(DEFAULT, Pad::new());
        let listener = HashMap::new();
        Self {
            fonts,
            colors,
            pads,
            listener,
        }
    }
}
