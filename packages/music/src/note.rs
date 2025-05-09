//! MIDI note

use core::fmt;

use crate::{Error, NoteName, Scale};

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(test, derive(Debug))]
pub struct Note(u8);

#[allow(non_upper_case_globals)]
impl Note {
    pub const A0: Self = Note(21);

    pub const G3: Self = Note(55);

    pub const C4: Self = Note(60);
    pub const Db4: Self = Note(61);
    pub const D4: Self = Note(62);
    pub const Eb4: Self = Note(63);
    pub const E4: Self = Note(64);
    pub const F4: Self = Note(65);
    pub const Gb4: Self = Note(66);
    pub const G4: Self = Note(67);
    pub const Ab4: Self = Note(68);
    pub const A4: Self = Note(69);
    pub const Bb4: Self = Note(70);
    pub const B4: Self = Note(71);

    pub const C5: Self = Note(72);
    pub const Db5: Self = Note(73);
    pub const D5: Self = Note(74);
    pub const Eb5: Self = Note(75);
    pub const E5: Self = Note(76);
    pub const F5: Self = Note(77);
    pub const Gb5: Self = Note(78);
    pub const G5: Self = Note(79);
    pub const Ab5: Self = Note(80);
    pub const A5: Self = Note(81);
    pub const Bb5: Self = Note(82);
    pub const B5: Self = Note(83);

    pub const C8: Self = Note(108);

    pub fn from_u8_lossy(value: u8) -> Self {
        Note(value & 0x7f)
    }

    pub fn is_natural(&self) -> bool {
        self.name().is_natural()
    }

    pub fn name(&self) -> NoteName {
        NoteName::from_u8_lossy(self.0 % 12)
    }

    pub fn octave(&self) -> i8 {
        self.0 as i8 / 12 - 1
    }

    pub fn distance_to(&self, other: Self) -> i8 {
        (other.0 as i8).wrapping_sub(self.0 as i8)
    }

    pub fn step(&self, half_steps: i8) -> Result<Note, Error> {
        let old = self.0 as i8;
        let new = old.checked_add(half_steps).ok_or(Error::NoteOutOfRange)?;
        if new < 0 {
            Err(Error::NoteOutOfRange)
        } else {
            Ok(Note(new as u8))
        }
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }

    pub fn display(&self, scale: Scale) -> impl fmt::Display {
        struct S {
            note: Note,
            scale: Scale,
        }

        impl fmt::Display for S {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "{}{}",
                    self.note.name().as_str(self.scale),
                    self.note.octave()
                )
            }
        }

        S { note: *self, scale }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_to() {
        assert_eq!(127, Note(0).distance_to(Note(127)));
        assert_eq!(-127, Note(127).distance_to(Note(0)));
    }
}
