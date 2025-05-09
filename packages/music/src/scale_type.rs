use crate::degree::Degree;

#[derive(Clone, Copy)]
pub enum ScaleType {
    Major,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Minor,
    HarmonicMinor,
}

impl ScaleType {
    pub const ALL: &'static [Self] = &[
        Self::Major,
        Self::Dorian,
        Self::Phrygian,
        Self::Lydian,
        Self::Mixolydian,
        Self::Minor,
        Self::HarmonicMinor,
    ];

    pub fn as_str(&self) -> &'static str {
        use ScaleType::*;

        match self {
            Major => "Major",
            Dorian => "Dorian",
            Phrygian => "Phrygian",
            Lydian => "Lydian",
            Mixolydian => "Mixolydian",
            Minor => "Minor",
            HarmonicMinor => "Harmonic Minor",
        }
    }

    pub fn degrees(&self) -> &'static [Degree] {
        use ScaleType::*;

        match self {
            Major => &Degree::IONIAN,
            Dorian => &Degree::DORIAN,
            Phrygian => &Degree::PHRYGIAN,
            Lydian => &Degree::LYDIAN,
            Mixolydian => &Degree::MIXOLYDIAN,
            Minor => &Degree::AEOLIAN,
            HarmonicMinor => &Degree::HARMONIC_MINOR,
        }
    }
}
