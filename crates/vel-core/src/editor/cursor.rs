
#[derive(Debug, Clone, PartialEq)]
pub struct Cursor {
    pos: usize,
    
    line: usize,
    col: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            pos: 0,
            line: 0,
            col: 0,
        }
    }
}