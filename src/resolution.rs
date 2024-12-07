use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Display for Resolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}×{}", self.width, self.height)
    }
}

impl Resolution {
    pub fn from_pair((width, height): (u32, u32)) -> Resolution {
        Resolution { width, height }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
}
