use crate::{Error, Note, Notes, notes};

/// A chord is two or more notes
#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Chord {
    notes: Notes,
}

impl Chord {
    /// Takes the lowest note and raises it up an octave
    pub fn invert_up(&mut self) -> Result<(), Error> {
        let lowest = self.lowest();
        let transposed = lowest.step(12)?;
        if self.contains(transposed) {
            return Err(Error::CannotInvert);
        }
        self.notes.insert(transposed);
        self.notes.remove(lowest);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    pub fn insert(&mut self, note: Note) {
        self.notes.insert(note);
    }

    pub fn contains(&self, note: Note) -> bool {
        self.notes.contains(note)
    }

    pub fn lowest(&self) -> Note {
        self.notes.lowest().unwrap()
    }

    pub fn highest(&self) -> Note {
        self.notes.highest().unwrap()
    }

    pub fn notes(&self) -> notes::Iter {
        self.notes.iter()
    }

    /// Takes the highest note and lowers it an octave
    pub fn invert_down(&mut self) -> Result<(), Error> {
        let highest = self.highest();
        let transposed = highest.step(-12)?;
        if self.contains(transposed) {
            return Err(Error::CannotInvert);
        }
        self.notes.insert(transposed);
        self.notes.remove(highest);
        Ok(())
    }
}

impl From<Chord> for Notes {
    fn from(chord: Chord) -> Self {
        chord.notes
    }
}

impl TryFrom<Notes> for Chord {
    type Error = ();

    fn try_from(notes: Notes) -> Result<Self, Self::Error> {
        if notes.len() < 2 {
            Err(())
        } else {
            Ok(Chord { notes })
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn invert_up() {
        let mut chord = chord![C4, E4, G4];
        let expected = chord![E4, G4, C5];
        chord.invert_up().unwrap();

        assert_eq!(expected, chord);
    }

    #[test]
    fn invert_down() {
        let mut chord = chord![C4, E4, G4];
        let expected = chord![G3, C4, E4];
        chord.invert_down().unwrap();

        assert_eq!(expected, chord);
    }
}
