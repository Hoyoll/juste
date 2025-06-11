use std::collections::HashSet;

pub enum Input {
    Combo(HashSet<On>),
    Single(On),
}

#[derive(PartialEq, Eq, Hash)]
pub enum On {
    Key(Button),
    Mouse(When),
}

#[derive(PartialEq, Eq, Hash)]
pub enum When {
    Hover,
    Moved(i32),
    Down(Mouse),
    Up(Mouse),
    Press(Mouse),
}

#[derive(PartialEq, Eq, Hash)]
pub enum Button {
    Down(Key),
    Up(Key),
    Press(Key),
}

#[derive(PartialEq, Eq, Hash)]
pub enum Mouse {
    Left,
    Right,
    Middle,
}

#[derive(PartialEq, Eq, Hash)]
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
