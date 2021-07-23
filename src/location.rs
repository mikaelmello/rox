use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Location {
    offset: usize,
    position: (usize, usize),
}

impl Location {
    pub fn ln(&mut self) {
        self.position = (self.position.0.saturating_add(1), 0);
    }

    pub fn advance(&mut self, c: u8) {
        self.offset = self.offset.saturating_add(1);
        self.position = (self.position.0, self.position.1.saturating_add(1));
        if c == b'\n' {
            self.ln();
        }
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl Display for Location {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{}:{}", self.position.0, self.position.1)
    }
}

impl From<(usize, usize, usize)> for Location {
    fn from((offset, line, col): (usize, usize, usize)) -> Self {
        Self {
            offset,
            position: (line, col),
        }
    }
}
