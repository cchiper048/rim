pub struct Line {
    data: String,
}

impl Line {
    pub fn new() -> Self {
        Self {
            data: String::from("")
        }
    }

    pub fn insert_char(&mut self, c: char, n: usize) -> &mut Self {
        self.data.insert(n, c);
        self
    }

    pub fn push_char(&mut self, c: char) -> &mut Self {
        self.data.push(c);
        self
    }

    pub fn remove_char_at(&mut self, index: usize) -> char {
        self.data.remove(index)
    }

    pub fn as_str(&self) -> &str {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn set_str(&mut self, s: &str) {
        self.data = s.to_string()
    }
}