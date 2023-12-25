use Requirement::{Any, MustBe};

#[derive(Copy, Clone, Debug)]
pub enum Requirement {
    MustBe(bool),
    Any,
}

impl Requirement {
    pub const fn equals(self, other: Self) -> bool {
        match self {
            Any => true,
            MustBe(x) => match other {
                Any => true,
                MustBe(y) => x == y,
            },
        }
    }
}
