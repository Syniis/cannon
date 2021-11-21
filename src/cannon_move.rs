use vek::Vec2;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CannonMove {
    pub source: Vec2<i32>,
    pub dest: Vec2<i32>,
    pub is_shot: bool,
}

impl CannonMove {
    pub fn new(source: Vec2<i32>, dest: Vec2<i32>, is_shot: bool) -> Self {
        Self {
            source,
            dest,
            is_shot,
        }
    }

    pub fn source(&self) -> Vec2<i32> {
        self.source
    }

    pub fn dest(&self) -> Vec2<i32> {
        self.dest
    }

    pub fn is_shot(&self) -> bool {
        self.is_shot
    }
}
