use Requirement::{Any, MustBe};

#[derive(Copy, Clone, Debug)]
pub enum Requirement {
    MustBe(bool),
    Any,
}

impl Requirement {
    pub fn equals(&self, other: &Requirement) -> bool {
        match self {
            Any => true,
            MustBe(x) => match other {
                Any => true,
                MustBe(y) => x == y,
            },
        }
    }
}
