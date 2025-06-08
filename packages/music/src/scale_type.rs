use crate::degree::Degree;

#[derive(Clone, Copy, PartialEq)]
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

    pub fn contains(&self, chord_tonic: Degree, is_minor_chord: bool) -> bool {
        use Degree::*;
        use ScaleType::*;

        matches!(
            (self, chord_tonic, is_minor_chord),
            (Major, One | Four | Five, false)
                | (Major, Two | Three | Six | Seven, true)
                | (Dorian, FlatThree | Five | FlatSeven, false)
                | (Phrygian, FlatTwo | FlatThree | FlatSix, false)
                | (Lydian, One | Two | Five, false)
                | (Mixolydian, One | Four | FlatSeven, false)
                | (Minor, FlatThree | FlatSix | FlatSeven, false)
                | (HarmonicMinor, FlatThree | Five, false)
        )
    }
}
