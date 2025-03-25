//! MIDI note

use core::fmt;

use crate::{NoteName, Scale};

#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Note(u8);

impl Note {
    pub const A0: Self = Note(21);
    pub const C4: Self = Note(60);
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

    pub fn step(&self, half_steps: i8) -> Option<Note> {
        let old = self.0 as i8;
        let new = old.checked_add(half_steps)?;
        if new < 0 { None } else { Some(Note(new as u8)) }
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
