use crate::{NoteName, ScaleType};

#[derive(Clone, Copy)]
pub struct Scale {
    pub tonic: NoteName,
    pub ty: ScaleType,
}

impl Scale {
    pub fn major(tonic: NoteName) -> Self {
        Self {
            tonic,
            ty: ScaleType::Major,
        }
    }

    pub fn minor(tonic: NoteName) -> Self {
        Self {
            tonic,
            ty: ScaleType::Minor,
        }
    }

    pub fn harmonic_minor(tonic: NoteName) -> Self {
        Self {
            tonic,
            ty: ScaleType::HarmonicMinor,
        }
    }
}
