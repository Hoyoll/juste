pub enum Input {
    Combo(Vec<On>),
    Single(On),
}

pub enum On {
    Key(Button),
    Mouse(When),
}

pub enum When {
    Hover,
    Down(Mouse),
    Up(Mouse),
    Press(Mouse),
}

pub enum Button {
    Down(Key),
    Up(Key),
    Press(Key),
}

pub enum Mouse {
    Left,
    Right,
    Middle,
}

pub enum Key {
    A,
    W,
    S,
    D,
}
