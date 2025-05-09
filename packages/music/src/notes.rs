use crate::Note;

const SIZE: usize = 128 / 8;

/// A collection of `Note`s
#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Notes {
    bits: [u8; SIZE],
}

impl Notes {
    pub const fn empty() -> Self {
        Self { bits: [0; SIZE] }
    }

    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|byte| *byte == 0)
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn clear(&mut self) {
        self.bits.iter_mut().for_each(|byte| *byte = 0);
    }

    pub fn contains(&self, note: Note) -> bool {
        let pos = note.as_u8() as usize;
        let mask = 1 << (pos % 8);
        self.bits[pos / 8] & mask != 0
    }

    pub fn lowest(&self) -> Option<Note> {
        self.iter().next()
    }

    pub fn highest(&self) -> Option<Note> {
        self.iter().last()
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut bits = [0; SIZE];
        for ((lhs, rhs), out) in self.bits.into_iter().zip(other.bits).zip(&mut bits) {
            *out = lhs & !rhs;
        }
        Self { bits }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut bits = [0; SIZE];
        for ((lhs, rhs), out) in self.bits.into_iter().zip(other.bits).zip(&mut bits) {
            *out = lhs | rhs;
        }
        Self { bits }
    }

    pub fn insert(&mut self, note: Note) {
        let pos = note.as_u8() as usize;
        self.bits[pos / 8] |= 1 << (pos % 8);
    }

    pub fn remove(&mut self, note: Note) {
        let pos = note.as_u8() as usize;
        self.bits[pos / 8] &= !(1 << (pos % 8));
    }

    pub fn iter(&self) -> Iter {
        Iter { bits: self.bits }
    }
}

impl FromIterator<Note> for Notes {
    fn from_iter<T: IntoIterator<Item = Note>>(iter: T) -> Self {
        let mut notes = Notes::empty();
        for note in iter {
            notes.insert(note);
        }
        notes
    }
}

impl IntoIterator for Notes {
    type Item = Note;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter {
    bits: [u8; SIZE],
}

impl Iterator for Iter {
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pos = 0;
        for byte in &mut self.bits {
            let tz = byte.trailing_zeros() as u8;

            pos += tz;
            if tz < 8 {
                *byte &= !(1 << tz);
                return Some(Note::from_u8_lossy(pos));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        let mut notes = Notes::empty();
        assert!(notes.iter().next().is_none());

        let c4 = Note::C4;
        let e4 = Note::E4;
        let g4 = Note::G4;

        notes.insert(g4);
        assert!(notes.iter().eq([g4]));

        notes.insert(e4);
        assert!(notes.iter().eq([e4, g4]));

        notes.insert(c4);
        assert!(notes.iter().eq([c4, e4, g4]));

        notes.remove(e4);
        assert!(notes.iter().eq([c4, g4]));

        notes.remove(c4);
        assert!(notes.iter().eq([g4]));

        notes.remove(g4);
        assert!(notes.iter().next().is_none());
    }

    #[test]
    fn union() {
        let c4 = Note::C4;
        let e4 = Note::E4;

        let mut lhs = Notes::empty();
        lhs.insert(c4);

        let mut rhs = Notes::empty();
        rhs.insert(e4);

        let result = lhs.union(&rhs);

        assert!(result.iter().eq([c4, e4]));
    }

    #[test]
    fn difference() {
        let lhs = notes![C4, E4];
        let rhs = notes![E4, G4];

        let l_minus_r = lhs.difference(&rhs);
        assert!(l_minus_r.iter().eq([Note::C4]));

        let r_minus_l = rhs.difference(&lhs);
        assert!(r_minus_l.iter().eq([Note::G4]));
    }
}
