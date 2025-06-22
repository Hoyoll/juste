use std::collections::HashSet;

use super::element::Code;

pub enum Input {
    Combo(HashSet<On>),
    Single(On),
    Custom(Code),
    None,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum On {
    Key(Button),
    Mouse(When),
    Window(Win),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Win {
    Resize { width: u32, height: u32 },
    Move { x: i32, y: i32 },
    Close,
    Cursor(Point),
    Scroll { delta: Delta, phase: Phase },
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Phase {
    Start,
    Move,
    End,
    Cancel,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Delta {
    Pixel { x: u32, y: u32 },
    Line { x: u32, y: u32 },
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Point {
    Enter,
    Left,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum When {
    Move { x: u32, y: u32 },
    Release(Mouse),
    Press(Mouse),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Button {
    Release(Key),
    Press(Key),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Mouse {
    Left,
    Right,
    Middle,
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
}
