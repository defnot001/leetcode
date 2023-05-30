struct MyHashSet {
    data: Vec<i32>,
}

impl MyHashSet {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn add(&mut self, key: i32) {
        if !self.data.contains(&key) {
            self.data.push(key);
        }
    }

    fn remove(&mut self, key: i32) {
        if let Some(pos) = self.data.iter().position(|x| *x == key) {
            self.data.remove(pos);
        }
    }

    fn contains(&self, key: i32) -> bool {
        if self.data.contains(&key) {
            return true;
        }

        false
    }
}
