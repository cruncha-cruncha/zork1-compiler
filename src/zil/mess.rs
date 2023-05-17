pub struct Huh {
    index: usize,
}

impl Huh {
    pub fn new() -> Huh {
        Huh { index: 0 }
    }
}

impl Iterator for Huh {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 10 {
            self.index += 1;
            return Some(self.index);
        } else {
            return None;
        }
    }
}
