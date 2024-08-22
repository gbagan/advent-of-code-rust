use crate::util::coord::Coord;

pub struct Box {
    pub xmin: i32,
    pub ymin: i32,
    pub xmax: i32,
    pub ymax: i32,
}

impl Box {
    #[inline]
    pub fn contains (&self, point: &Coord) -> bool {
        self.xmin <= point.x && point.x <= self.xmax && self.ymin <= point.y && point.y <= self.ymax
    }
}