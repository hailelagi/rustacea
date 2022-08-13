#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

