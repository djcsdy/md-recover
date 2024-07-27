use std::fmt::{Display, Formatter};

pub struct Confidence {
    good: usize,
    total: usize,
}

impl Confidence {
    pub fn good(&self) -> usize {
        self.good
    }

    pub fn bad(&self) -> usize {
        self.total - self.good
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub fn as_ratio(&self) -> f64 {
        self.good as f64 / self.total as f64
    }

    pub fn as_percentage(&self) -> f64 {
        self.as_ratio() * 100.0
    }
}

impl Display for Confidence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.as_percentage())
    }
}
