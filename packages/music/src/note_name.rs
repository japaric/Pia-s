use crate::{Interval, Scale};

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(Debug, Eq, Ord, PartialOrd))]
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

    pub fn distance_to(&self, other: Self) -> Interval {
        Interval::from_i8_lossy(other as i8 - *self as i8)
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

        match (scale.tonic(), self) {
            (C, Gb)
            | (G, Gb | Db)
            | (D, Gb | Db | Ab)
            | (A, Gb | Db | Ab | Eb)
            | (E | B, Gb | Db | Ab | Eb | Bb) => self.as_sharp_str(),

            _ => self.as_flat_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Degree;

    use super::*;

    #[test]
    fn sharp_four() {
        for tonic in NoteName::CIRCLE_OF_FIFTHS {
            let scale = Scale::new(tonic);
            let note = scale.degree2name(Degree::SharpFour);

            dbg!(scale, note);
            assert!(!note.as_str(scale).contains('♭'));
        }
    }

    #[test]
    fn sharp_keys() {
        use NoteName::*;

        let cases: [(_, &[_]); 5] = [
            (G, &[Gb]),
            (D, &[Gb, Db]),
            (A, &[Gb, Db, Ab]),
            (E, &[Gb, Db, Ab, Eb]),
            (B, &[Gb, Db, Ab, Eb, Bb]),
        ];

        for (tonic, notes) in cases {
            let scale = Scale::new(tonic);

            for note in notes {
                assert!(note.as_str(scale).contains('♯'));
            }
        }
    }
}
