#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: isize,
    pub y: isize
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Pos { x: x, y: y }
    }
}
