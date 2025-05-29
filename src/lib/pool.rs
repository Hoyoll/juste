pub struct Pool<T> {
    pub items: Vec<T>,
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, item: T) {
        self.items.push_back(item)
    }
}
