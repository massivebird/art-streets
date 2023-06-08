use crate::tile::requirement::Requirement;

#[derive(Copy, Clone, Debug)]
pub struct Constraint {
    pub up: Requirement,
    pub right: Requirement,
    pub down: Requirement,
    pub left: Requirement,
}

impl Constraint {
    pub fn equals(&self, other: &Constraint) -> bool {
        self.up.equals(&other.up)
            && self.right.equals(&other.right)
            && self.down.equals(&other.down)
            && self.left.equals(&other.left)
    }
}
