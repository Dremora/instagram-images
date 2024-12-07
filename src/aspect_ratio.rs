use std::fmt;

use clap::ValueEnum;

use crate::Resolution;

#[derive(Clone, Copy, ValueEnum)]
pub enum AspectRatio {
    Portrait,
    Landscape,
    Square,
}

impl fmt::Display for AspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AspectRatio::Portrait => write!(f, "portrait"),
            AspectRatio::Landscape => write!(f, "landscape"),
            AspectRatio::Square => write!(f, "square"),
        }
    }
}

impl AspectRatio {
    pub fn to_resolution(&self) -> Resolution {
        match self {
            AspectRatio::Portrait => Resolution {
                width: 1080,
                height: 1350,
            },
            AspectRatio::Landscape => Resolution {
                width: 1080,
                height: 566,
            },
            AspectRatio::Square => Resolution {
                width: 1080,
                height: 1080,
            },
        }
    }
}
