use crate::Degree;

const SIZE: usize = 2;

#[derive(Clone)]
pub struct Degrees {
    bits: [u8; SIZE],
}

impl Degrees {
    pub fn empty() -> Self {
        Self { bits: [0; SIZE] }
    }

    pub fn contains(&self, degree: Degree) -> bool {
        let pos = degree as usize;
        let mask = 1 << (pos % 8);
        self.bits[pos / 8] & mask != 0
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|byte| *byte == 0)
    }

    pub fn iter(&self) -> Iter {
        Iter { bits: self.bits }
    }

    pub fn insert(&mut self, degree: Degree) {
        let pos = degree as usize;
        self.bits[pos / 8] |= 1 << (pos % 8);
    }

    pub fn remove(&mut self, degree: Degree) {
        let pos = degree as usize;
        self.bits[pos / 8] &= !(1 << (pos % 8));
    }
}

impl IntoIterator for Degrees {
    type Item = Degree;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter {
    bits: [u8; SIZE],
}

impl Iterator for Iter {
    type Item = Degree;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pos = 0;
        for byte in &mut self.bits {
            let tz = byte.trailing_zeros() as u8;

            pos += tz;
            if tz < 8 {
                *byte &= !(1 << tz);
                return Some(Degree::from_u8_lossy(pos));
            }
        }

        None
    }
}
