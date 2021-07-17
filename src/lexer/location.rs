use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Location(usize, usize);

impl Location {
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
        write!(fmt, "{}:{}", self.0, self.1)
    }
}

impl From<(usize, usize)> for Location {
    fn from(v: (usize, usize)) -> Self {
        Self(v.0, v.1)
    }
}
