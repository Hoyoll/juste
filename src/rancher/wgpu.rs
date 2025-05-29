use std::collections::HashSet;

use super::io::Input;

enum IO {
    Zero,
    One(Input),
    More(HashSet<Input>),
}

pub struct Renderer {}
