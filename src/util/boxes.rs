use crate::util::coord::Coord;

pub struct Box {
    pub xmin: i64,
    pub ymin: i64,
    pub xmax: i64,
    pub ymax: i64,
}

impl Box {
    #[inline]
    pub fn contains (&self, point: &Coord) -> bool {
        self.xmin <= point.x && point.x <= self.xmax && self.ymin <= point.y && point.y <= self.ymax
    }
}