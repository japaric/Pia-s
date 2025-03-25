use crate::{Degree, NoteName};

#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug))]
pub struct Scale {
    tonic: NoteName,
}

impl Scale {
    pub fn new(tonic: NoteName) -> Self {
        Self { tonic }
    }

    pub fn degree2name(&self, degree: Degree) -> NoteName {
        NoteName::from_u8_lossy(self.tonic as u8 + degree as u8)
    }

    pub fn name2degree(&self, name: NoteName) -> Degree {
        Degree::from_u8_lossy(12 + name as u8 - self.tonic as u8)
    }

    pub fn tonic(&self) -> NoteName {
        self.tonic
    }
}
