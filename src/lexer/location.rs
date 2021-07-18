use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Location(isize, isize);

impl Location {
    pub const EOF: Location = Location(-1, -1);

    pub fn lf(&mut self) {
        self.0 = self.0.saturating_add(1);
    }

    pub fn cr(&mut self) {
        self.1 = 0;
    }

    pub fn next(&mut self) {
        self.1 = self.1.saturating_add(1);
    }
}

impl Display for Location {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if self.0 < 0 {
            write!(fmt, "EOF")
        } else {
            write!(fmt, "{}:{}", self.0, self.1)
        }
    }
}

impl From<(isize, isize)> for Location {
    fn from(v: (isize, isize)) -> Self {
        Self(v.0, v.1)
    }
}
