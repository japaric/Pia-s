use crate::ScaleType;

#[derive(Clone, Copy, PartialEq)]
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

    pub fn belongs_to(&self, scale: ScaleType) -> bool {
        scale.degrees().contains(self)
    }
}
