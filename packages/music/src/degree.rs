use crate::ScaleType;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(test, derive(Debug))]
pub enum Degree {
    One,
    FlatTwo,
    Two,
    FlatThree,
    Three,
    Four,
    SharpFour,
    Five,
    FlatSix,
    Six,
    FlatSeven,
    Seven,
}

impl Degree {
    pub const ALL: [Self; 12] = [
        Self::One,
        Self::FlatTwo,
        Self::Two,
        Self::FlatThree,
        Self::Three,
        Self::Four,
        Self::SharpFour,
        Self::Five,
        Self::FlatSix,
        Self::Six,
        Self::FlatSeven,
        Self::Seven,
    ];

    pub const IONIAN: [Self; 7] = [
        Self::One,
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::Seven,
    ];

    pub const DORIAN: [Self; 7] = [
        Self::One,
        Self::Two,
        Self::FlatThree,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::FlatSeven,
    ];

    pub const PHRYGIAN: [Self; 7] = [
        Self::One,
        Self::FlatTwo,
        Self::FlatThree,
        Self::Four,
        Self::Five,
        Self::FlatSix,
        Self::FlatSeven,
    ];

    pub const LYDIAN: [Self; 7] = [
        Self::One,
        Self::Two,
        Self::Three,
        Self::SharpFour,
        Self::Five,
        Self::Six,
        Self::Seven,
    ];

    pub const MIXOLYDIAN: [Self; 7] = [
        Self::One,
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::FlatSeven,
    ];

    pub const AEOLIAN: [Self; 7] = [
        Self::One,
        Self::Two,
        Self::FlatThree,
        Self::Four,
        Self::Five,
        Self::FlatSix,
        Self::FlatSeven,
    ];

    pub const HARMONIC_MINOR: [Self; 7] = [
        Self::One,
        Self::Two,
        Self::FlatThree,
        Self::Four,
        Self::Five,
        Self::FlatSix,
        Self::Seven,
    ];

    pub(crate) fn from_u8_lossy(value: u8) -> Self {
        use Degree::*;

        match value % 12 {
            0 => One,
            1 => FlatTwo,
            2 => Two,
            3 => FlatThree,
            4 => Three,
            5 => Four,
            6 => SharpFour,
            7 => Five,
            8 => FlatSix,
            9 => Six,
            10 => FlatSeven,
            _ => Seven,
        }
    }

    pub fn as_str(&self) -> &'static str {
        use Degree::*;

        match self {
            One => "1",
            FlatTwo => "♭2",
            Two => "2",
            FlatThree => "♭3",
            Three => "3",
            Four => "4",
            SharpFour => "♯4",
            Five => "5",
            FlatSix => "♭6",
            Six => "6",
            FlatSeven => "♭7",
            Seven => "7",
        }
    }

    pub fn roman_major(&self) -> &'static str {
        use Degree::*;

        match self {
            One => "Ⅰ",
            FlatTwo => "♭Ⅱ",
            Two => "Ⅱ",
            FlatThree => "♭Ⅲ",
            Three => "Ⅲ",
            Four => "Ⅳ",
            SharpFour => "♯Ⅳ",
            Five => "Ⅴ",
            FlatSix => "♭Ⅵ",
            Six => "Ⅵ",
            FlatSeven => "♭Ⅶ",
            Seven => "Ⅶ",
        }
    }

    pub fn roman_minor(&self) -> &'static str {
        use Degree::*;

        match self {
            One => "ⅰ",
            FlatTwo => "♭ⅱ",
            Two => "ⅱ",
            FlatThree => "♭ⅲ",
            Three => "ⅲ",
            Four => "ⅳ",
            SharpFour => "♯ⅳ",
            Five => "ⅴ",
            FlatSix => "♭ⅵ",
            Six => "ⅵ",
            FlatSeven => "♭ⅶ",
            Seven => "ⅶ",
        }
    }

    pub fn belongs_to(&self, scale: ScaleType) -> bool {
        scale.degrees().contains(self)
    }

    pub fn step(&self, half_steps: isize) -> Self {
        let prev_index = *self as isize;
        let mut new_index = (prev_index + (half_steps % 12)) % 12;
        if new_index < 0 {
            new_index += 12;
        }
        Self::from_u8_lossy(new_index as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step() {
        use Degree::*;

        assert_eq!(Five, One.step(7));
        assert_eq!(Four, One.step(-7));

        assert_eq!(FlatThree, One.step(3));
        assert_eq!(Three, One.step(4));

        assert_eq!(Six, One.step(-3));
        assert_eq!(FlatSix, One.step(-4));
    }
}
