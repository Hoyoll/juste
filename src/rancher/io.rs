pub enum Input {
    Combo(Vec<From>),
    Single(From),
}

pub enum From {
    Key(Key),
    Mouse(Mouse),
}

pub enum Mouse {
    Hover,
    Down(MAtom),
    Up(MAtom),
    Press(MAtom),
}

pub enum Key {
    Down(KAtom),
    Up(KAtom),
    Press(KAtom),
}

pub enum MAtom {
    LeftMouse,
    RightMouse,
    MiddleMouse,
}
pub enum KAtom {
    A,
    W,
    S,
    D,
}
