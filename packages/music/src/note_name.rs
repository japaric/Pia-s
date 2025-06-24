use crate::{Error, Interval, Note, ScaleType, scale::Scale};

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(test, derive(Debug))]
pub enum NoteName {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

impl NoteName {
    pub const CIRCLE_OF_FIFTHS: [Self; 12] = [
        Self::Gb,
        Self::Db,
        Self::Ab,
        Self::Eb,
        Self::Bb,
        Self::F,
        Self::C,
        Self::G,
        Self::D,
        Self::A,
        Self::E,
        Self::B,
    ];

    pub(crate) fn from_u8_lossy(value: u8) -> Self {
        use NoteName::*;

        match value % 12 {
            0 => C,
            1 => Db,
            2 => D,
            3 => Eb,
            4 => E,
            5 => F,
            6 => Gb,
            7 => G,
            8 => Ab,
            9 => A,
            10 => Bb,
            _ => B,
        }
    }

    pub fn with_octave(&self, octave: i8) -> Result<Note, Error> {
        if !(0..=8).contains(&octave) {
            return Err(Error::NoteOutOfRange);
        }

        if (octave == 8 && *self > NoteName::C) || (octave == 0 && *self < NoteName::A) {
            return Err(Error::NoteOutOfRange);
        }

        Ok(Note::from_u8_lossy(
            ((*self as i8) + 12 * (octave + 1)) as u8,
        ))
    }

    pub fn distance_to(&self, other: Self) -> Interval {
        Interval::from_u8_lossy((other as i8 - *self as i8).unsigned_abs())
    }

    pub fn step(&self, half_steps: u8) -> NoteName {
        Self::from_u8_lossy(*self as u8 + half_steps)
    }

    pub fn is_natural(&self) -> bool {
        use NoteName::*;

        matches!(self, C | D | E | F | G | A | B)
    }

    pub fn as_flat_str(&self) -> &'static str {
        use NoteName::*;

        match self {
            C => "C",
            Db => "D♭",
            D => "D",
            Eb => "E♭",
            E => "E",
            F => "F",
            Gb => "G♭",
            G => "G",
            Ab => "A♭",
            A => "A",
            Bb => "B♭",
            B => "B",
        }
    }

    pub fn as_sharp_str(&self) -> &'static str {
        use NoteName::*;

        match self {
            C => "C",
            Db => "C♯",
            D => "D",
            Eb => "D♯",
            E => "E",
            F => "F",
            Gb => "F♯",
            G => "G",
            Ab => "G♯",
            A => "A",
            Bb => "A♯",
            B => "B",
        }
    }

    pub fn as_str(&self, scale: Scale) -> &'static str {
        use NoteName::*;

        if matches!(scale.ty, ScaleType::HarmonicMinor) {
            match (self, scale.tonic) {
                (Gb, G) | (Db, D) | (Ab, A) | (Eb, E) | (Bb, B) => {
                    return self.as_sharp_str();
                }

                (B, Ab) => return "C♭",
                (E, Ab) => return "F♭",
                (C, Db) => return "B♯",
                (F, Gb) => return "E♯",

                (_, Ab) => return self.as_flat_str(),

                _ => {}
            }
        }

        let offset = match scale.ty {
            ScaleType::Major => 0,
            ScaleType::Dorian => 10,
            ScaleType::Phrygian => 8,
            ScaleType::Lydian => 7,
            ScaleType::Mixolydian => 5,
            ScaleType::Minor | ScaleType::HarmonicMinor => 3,
        };
        let relative_tonic = scale.tonic.step(offset);
        match (relative_tonic, self) {
            (C, Gb)
            | (G, Gb | Db)
            | (D, Gb | Db | Ab)
            | (A, Gb | Db | Ab | Eb)
            | (E | B, Gb | Db | Ab | Eb | Bb) => self.as_sharp_str(),

            (Gb, B) => "C♭",

            _ => self.as_flat_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Degree, MajorScale};

    use super::*;

    #[test]
    fn sharp_four() {
        for tonic in NoteName::CIRCLE_OF_FIFTHS {
            let scale = MajorScale::new(tonic);
            let note = scale.degree2name(Degree::SharpFour);

            dbg!(scale, note);
            assert!(!note.as_str(Scale::major(tonic)).contains('♭'));
        }
    }

    #[test]
    fn sharp_notes() {
        use NoteName::*;

        let cases: [(_, &[_]); 5] = [
            (G, &[Gb]),
            (D, &[Gb, Db]),
            (A, &[Gb, Db, Ab]),
            (E, &[Gb, Db, Ab, Eb]),
            (B, &[Gb, Db, Ab, Eb, Bb]),
        ];

        for (tonic, notes) in cases {
            let scale = Scale::major(tonic);

            for note in notes {
                assert!(note.as_str(scale).contains('♯'));
            }
        }
    }

    #[test]
    fn b_in_flat_g_major() {
        use NoteName::*;

        assert_eq!("C♭", B.as_str(Scale::major(Gb)));
    }

    #[test]
    fn minor_scales() {
        use NoteName::*;

        let stringify = |name: NoteName| name.as_str(Scale::minor(name));

        assert_eq!("F♯", stringify(Gb));
        assert_eq!("C♯", stringify(Db));
        assert_eq!("G♯", stringify(Ab));
        assert_eq!("E♭", stringify(Eb));
        assert_eq!("B♭", stringify(Bb));
    }

    #[test]
    fn harmonic_minor_is_fun() {
        use NoteName::*;

        let stringify = |name: NoteName, tonic: NoteName| name.as_str(Scale::harmonic_minor(tonic));

        assert_eq!("F♯", stringify(Gb, G));
        assert_eq!("C♯", stringify(Db, D));
        assert_eq!("G♯", stringify(Ab, A));
        assert_eq!("D♯", stringify(Eb, E));
        assert_eq!("A♯", stringify(Bb, B));
        assert_eq!("C♭", stringify(B, Eb));
        assert_eq!("C♭", stringify(B, Ab));
        assert_eq!("F♭", stringify(E, Ab));
        assert_eq!("B♯", stringify(C, Db));
        assert_eq!("E♯", stringify(F, Gb));

        assert_eq!("A♭", stringify(Ab, Ab));
    }

    #[test]
    fn with_octave() {
        assert_eq!(Ok(Note::A0), NoteName::A.with_octave(0));
        assert_eq!(Ok(Note::C8), NoteName::C.with_octave(8));

        assert_eq!(Err(Error::NoteOutOfRange), NoteName::Ab.with_octave(0));
        assert_eq!(Err(Error::NoteOutOfRange), NoteName::Db.with_octave(8));
    }
}
