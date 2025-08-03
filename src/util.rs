#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct GapBuf<T: Clone> {
    pub left: Vec<T>,
    pub right: Vec<T>,
}

pub enum BufError {
    BufEnd { overshoot: usize },
    BufStar { undershoot: usize },
}

impl<T> GapBuf<T> {
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    pub fn insert(&mut self, token: T) {
        self.left.push(token);
    }

    pub fn pop_left(&mut self) -> Option<T> {
        self.left.pop()
    }

    pub fn pop_right(&mut self) -> Option<T> {
        self.right.pop()
    }

    pub fn peek_left(&mut self) -> Option<&mut T> {
        self.left.last_mut()
    }

    pub fn peek_right(&mut self) -> Option<&mut T> {
        self.right.last_mut()
    }

    pub fn current_gap(&self) -> usize {
        self.left.len()
    }

    pub fn shift_weight_left(&mut self) {
        while let Some(token) = self.right.pop() {
            self.left.push(token);
        }
    }

    pub fn shift_weight_right(&mut self) {
        while let Some(token) = self.left.pop() {
            self.right.push(token);
        }
    }

    pub fn weight(&mut self) -> usize {
        (self.left.len() + self.right.len())
    }

    pub fn shift_left(&mut self, amount: usize) -> Result<(), BufError> {
        for i in 0..amount {
            if let Some(token) = self.left.pop() {
                self.right.push(token);
            } else {
                return Err(BufError { undershoot: i });
            }
        }
        Ok(())
    }

    pub fn shift_right(&mut self, amount: usize) -> Result<(), BufError> {
        for i in 0..amount {
            if let Some(token) = self.right.pop() {
                self.left.push(token);
            } else {
                return Err(BufError { overshoot: i });
            }
        }
        Ok(())
    }

    pub fn iter_mut<F>(&mut self, mut fun: F)
    where
        F: FnMut(&mut T),
    {
        self.left.iter_mut().for_each(&mut fun);
        self.right.iter_mut().rev().for_each(&mut fun);
    }

    pub fn iter<F>(&mut self, fun: F)
    where
        F: FnMut(&T),
    {
        self.left.iter().for_each(fun);
        self.right.iter().rev().for_each(fun);
    }

    pub fn collect(&self) -> Vec<T> {
        let mut vec = Vec::new();
        self.left.iter().for_each(|t| {
            vec.push(t.clone());
        });
        self.right.iter().rev().for_each(|t| {
            vec.push(t.clone());
        });
        vec
    }
}
