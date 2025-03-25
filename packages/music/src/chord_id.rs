use core::fmt::{self, Write as _};
#[cfg(test)]
use std::collections::{BTreeMap, BTreeSet};

use crate::{Chord, Note, NoteName, Notes, Scale, note_names::NoteNames};

impl Chord {
    pub fn identify_with_tonic(&self, tonic: NoteName) -> Option<ChordId> {
        let lowest = self.lowest();

        let mut names = NoteNames::empty();
        let mut no_duplicates = Notes::empty();

        for note in self.notes() {
            let name = note.name();
            if !names.contains(name) {
                no_duplicates.insert(note);
                names.insert(name);
            }
        }

        let tonic_note = find(&no_duplicates, tonic);
        if self
            .notes()
            .filter(|note| note.name() == lowest.name())
            .all(|note| note.distance_to(tonic_note) >= 24)
        {
            // heuristic: tonic cannot be that low
            return None;
        }

        'next_kind: for &kind in ChordKind::ALL {
            let mut remaining = no_duplicates.clone();
            rm(&mut remaining, tonic);

            let minor_second = tonic.step(1);
            let major_second = tonic.step(2);
            let minor_third = tonic.step(3);
            let major_third = tonic.step(4);
            let perfect_fourth = tonic.step(5);
            let flat_fifth = tonic.step(6);
            let perfect_fifth = tonic.step(7);
            let sharp_fifth = tonic.step(8);
            let major_sixth = tonic.step(9);
            let minor_seventh = tonic.step(10);
            let major_seventh = tonic.step(11);

            let kind = match kind {
                ChordKind::Power => {
                    if names.contains(perfect_fifth) && names.len() == 2 {
                        rm(&mut remaining, perfect_fifth);

                        kind
                    } else {
                        continue;
                    }
                }

                ChordKind::Major => {
                    if names.contains(major_third)
                        && !names.contains(major_sixth)
                        && !names.contains(minor_seventh)
                        && !names.contains(major_seventh)
                    {
                        rm(&mut remaining, major_third);

                        if names.contains(perfect_fifth) {
                            rm(&mut remaining, perfect_fifth);

                            kind
                        } else if !names.is_empty() && !names.contains(sharp_fifth) {
                            kind
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                ChordKind::Minor => {
                    if names.contains(minor_third)
                        && names.contains(perfect_fifth)
                        && !names.contains(major_sixth)
                        && !names.contains(minor_seventh)
                        && !names.contains(major_seventh)
                    {
                        rm(&mut remaining, minor_third);
                        rm(&mut remaining, perfect_fifth);

                        kind
                    } else {
                        continue;
                    }
                }

                ChordKind::Sus2 => {
                    if names.contains(major_second)
                        && names.contains(perfect_fifth)
                        && !names.contains(minor_third)
                        && !names.contains(major_third)
                        && !names.contains(minor_seventh)
                        && !names.contains(major_seventh)
                        && (!names.contains(perfect_fourth)
                            || find(&no_duplicates, perfect_fourth)
                                > find(&no_duplicates, major_second))
                    {
                        rm(&mut remaining, major_second);
                        rm(&mut remaining, perfect_fifth);

                        kind
                    } else {
                        continue;
                    }
                }

                ChordKind::Sus4 => {
                    if names.contains(perfect_fourth)
                        && names.contains(perfect_fifth)
                        && !names.contains(minor_third)
                        && !names.contains(major_third)
                        && !names.contains(minor_seventh)
                        && !names.contains(major_seventh)
                        && (!names.contains(major_second)
                            || find(&no_duplicates, major_second)
                                > find(&no_duplicates, perfect_fourth))
                    {
                        rm(&mut remaining, perfect_fourth);
                        rm(&mut remaining, perfect_fifth);

                        kind
                    } else {
                        continue;
                    }
                }

                ChordKind::Augmented => {
                    if names.contains(major_third)
                        && names.contains(sharp_fifth)
                        && !names.contains(minor_seventh)
                        && !names.contains(major_seventh)
                    {
                        rm(&mut remaining, major_third);
                        rm(&mut remaining, sharp_fifth);

                        kind
                    } else {
                        continue;
                    }
                }

                ChordKind::Diminished => {
                    if names.contains(minor_third)
                        && names.contains(flat_fifth)
                        && !names.contains(major_sixth)
                        && !names.contains(minor_seventh)
                        && !names.contains(major_seventh)
                    {
                        rm(&mut remaining, minor_third);
                        rm(&mut remaining, flat_fifth);

                        kind
                    } else {
                        continue;
                    }
                }

                ChordKind::Lydian => {
                    if names.contains(flat_fifth)
                        && names.contains(perfect_fifth)
                        && names.len() == 3
                    {
                        rm(&mut remaining, flat_fifth);
                        rm(&mut remaining, perfect_fifth);

                        kind
                    } else {
                        continue;
                    }
                }

                ChordKind::Phrygian => {
                    if names.contains(minor_second)
                        && names.contains(perfect_fifth)
                        && names.len() == 3
                    {
                        rm(&mut remaining, minor_second);
                        rm(&mut remaining, perfect_fifth);

                        kind
                    } else {
                        continue;
                    }
                }

                ChordKind::MajorSixth => {
                    if names.contains(major_third)
                        && names.contains(perfect_fifth)
                        && names.contains(major_sixth)
                        && !names.contains(minor_seventh)
                        && !names.contains(major_seventh)
                    {
                        rm(&mut remaining, major_third);
                        rm(&mut remaining, perfect_fifth);
                        rm(&mut remaining, major_sixth);

                        kind
                    } else {
                        continue;
                    }
                }

                ChordKind::MinorSixth => {
                    if names.contains(minor_third)
                        && names.contains(perfect_fifth)
                        && names.contains(major_sixth)
                        && !names.contains(minor_seventh)
                        && !names.contains(major_seventh)
                    {
                        rm(&mut remaining, minor_third);
                        rm(&mut remaining, perfect_fifth);
                        rm(&mut remaining, major_sixth);

                        kind
                    } else {
                        continue;
                    }
                }

                ChordKind::MinorMajorSeventh(_) => {
                    if names.contains(minor_third) && names.contains(major_seventh) {
                        rm(&mut remaining, minor_third);
                        rm(&mut remaining, major_seventh);

                        let alteration = if names.contains(perfect_fifth) {
                            rm(&mut remaining, perfect_fifth);

                            Alteration::None
                        } else if names.contains(sharp_fifth) {
                            rm(&mut remaining, sharp_fifth);

                            Alteration::SharpFive
                        } else {
                            Alteration::None
                        };

                        ChordKind::MinorMajorSeventh(alteration)
                    } else {
                        continue;
                    }
                }

                ChordKind::MajorSeventh(_) => {
                    if names.contains(major_seventh) && !names.contains(minor_third) {
                        rm(&mut remaining, major_seventh);

                        let alteration = if names.contains(perfect_fifth) {
                            rm(&mut remaining, perfect_fifth);

                            if names.contains(major_third) {
                                rm(&mut remaining, major_third);

                                Alteration::None
                            } else if names.contains(major_second) {
                                rm(&mut remaining, major_second);

                                Alteration::Sus2
                            } else if names.contains(perfect_fourth) {
                                rm(&mut remaining, perfect_fourth);

                                Alteration::Sus4
                            } else {
                                Alteration::NoThree
                            }
                        } else if names.contains(major_third) {
                            rm(&mut remaining, major_third);

                            if names.contains(sharp_fifth) {
                                rm(&mut remaining, sharp_fifth);

                                Alteration::SharpFive
                            } else if names.contains(flat_fifth) {
                                rm(&mut remaining, flat_fifth);

                                Alteration::FlatFive
                            } else {
                                Alteration::None
                            }
                        } else {
                            continue;
                        };

                        ChordKind::MajorSeventh(alteration)
                    } else {
                        continue;
                    }
                }

                ChordKind::MinorSeventh(_) => {
                    if names.contains(minor_third) && names.contains(minor_seventh) {
                        rm(&mut remaining, minor_third);
                        rm(&mut remaining, minor_seventh);

                        let alteration = if names.contains(perfect_fifth) {
                            rm(&mut remaining, perfect_fifth);
                            Alteration::None
                        } else if names.contains(flat_fifth) {
                            rm(&mut remaining, flat_fifth);
                            Alteration::FlatFive
                        } else if names.contains(sharp_fifth) {
                            rm(&mut remaining, sharp_fifth);
                            Alteration::SharpFive
                        } else {
                            Alteration::None
                        };

                        ChordKind::MinorSeventh(alteration)
                    } else {
                        continue;
                    }
                }

                ChordKind::Seventh(_) => {
                    if names.contains(minor_seventh) && !names.contains(minor_third) {
                        rm(&mut remaining, minor_seventh);

                        let alteration = if names.contains(perfect_fifth) {
                            rm(&mut remaining, perfect_fifth);

                            if names.contains(major_third) {
                                rm(&mut remaining, major_third);

                                Alteration::None
                            } else if names.contains(perfect_fourth) {
                                rm(&mut remaining, perfect_fourth);

                                Alteration::Sus4
                            } else if names.contains(major_second) {
                                rm(&mut remaining, major_second);

                                Alteration::Sus2
                            } else {
                                Alteration::NoThree
                            }
                        } else if names.contains(major_third) {
                            rm(&mut remaining, major_third);

                            if names.contains(sharp_fifth) {
                                rm(&mut remaining, sharp_fifth);

                                Alteration::SharpFive
                            } else if names.contains(flat_fifth) {
                                rm(&mut remaining, flat_fifth);

                                Alteration::FlatFive
                            } else {
                                Alteration::None
                            }
                        } else if names.contains(perfect_fourth) {
                            rm(&mut remaining, perfect_fourth);

                            // avoid labelling three note chords as sevenths
                            if remaining.is_empty() {
                                continue;
                            }

                            Alteration::Sus4
                        } else if names.contains(major_second) {
                            rm(&mut remaining, major_second);

                            // avoid labelling three note chords as sevenths
                            if remaining.is_empty() {
                                continue;
                            }

                            Alteration::Sus2
                        } else {
                            continue;
                        };

                        ChordKind::Seventh(alteration)
                    } else {
                        continue;
                    }
                }

                ChordKind::DiminishedSeventh => {
                    if names.contains(minor_third)
                        && names.contains(flat_fifth)
                        && names.contains(major_sixth)
                        && !names.contains(major_third)
                        && !names.contains(perfect_fifth)
                        && !names.contains(minor_seventh)
                        && !names.contains(major_seventh)
                    {
                        rm(&mut remaining, minor_third);
                        rm(&mut remaining, flat_fifth);
                        rm(&mut remaining, major_sixth);

                        kind
                    } else {
                        continue;
                    }
                }
            };

            let bass = if lowest.name() == tonic {
                None
            } else {
                Some(lowest.name())
            };

            // heuristics: only allowed in root position
            if matches!(
                kind,
                ChordKind::Augmented
                    | ChordKind::DiminishedSeventh
                    | ChordKind::MajorSixth
                    | ChordKind::MinorSixth
            ) && bass.is_some()
            {
                continue;
            }

            let mut extensions = Extensions::none();
            for note in remaining {
                if !extensions.add(tonic_note, note) {
                    continue 'next_kind;
                }
            }

            let chord_id = ChordId {
                bass,
                kind,
                extensions,
            };

            return Some(chord_id);
        }

        None
    }

    #[cfg(test)]
    fn identify(&self) -> BTreeMap<NoteName, ChordId> {
        let names: BTreeSet<_> = self.notes().map(|note| note.name()).collect();

        let mut map = BTreeMap::new();
        for tonic in names {
            if let Some(chord_id) = self.identify_with_tonic(tonic) {
                map.insert(tonic, chord_id);
            }
        }

        map
    }
}

fn find(notes: &Notes, name: NoteName) -> Note {
    notes.iter().find(|note| note.name() == name).unwrap()
}

fn rm(notes: &mut Notes, name: NoteName) {
    let note = find(notes, name);
    notes.remove(note);
}

#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct ChordId {
    pub bass: Option<NoteName>,
    pub kind: ChordKind,
    pub extensions: Extensions,
}

impl ChordId {
    pub fn sup(&self) -> impl fmt::Display {
        struct S(ChordId);

        impl fmt::Display for S {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut extensions = self.0.extensions;
                let (s, alteration, collapse_extensions) = match self.0.kind {
                    ChordKind::Power => ("5", Alteration::None, false),

                    ChordKind::Augmented | ChordKind::Major | ChordKind::Minor => {
                        ("", Alteration::None, false)
                    }

                    ChordKind::Sus2 => ("sus2", Alteration::None, false),

                    ChordKind::Sus4 => ("sus4", Alteration::None, false),

                    ChordKind::Diminished => ("o", Alteration::None, false),

                    ChordKind::Lydian => ("Lyd", Alteration::None, false),

                    ChordKind::Phrygian => ("Phr", Alteration::None, false),

                    ChordKind::MajorSixth | ChordKind::MinorSixth => ("6", Alteration::None, false),

                    ChordKind::MajorSeventh(alteration)
                    | ChordKind::MinorMajorSeventh(alteration) => {
                        let s = if extensions.ninth == Some(Accidental::Natural) {
                            extensions.ninth = None;
                            "Δ9"
                        } else {
                            "Δ"
                        };

                        (s, alteration, false)
                    }

                    ChordKind::MinorSeventh(alteration) | ChordKind::Seventh(alteration) => {
                        let s =
                            if matches!(self.0.kind, ChordKind::MinorSeventh(Alteration::FlatFive))
                            {
                                "ø"
                            } else {
                                ""
                            };

                        (s, alteration, true)
                    }

                    ChordKind::DiminishedSeventh => ("o", Alteration::None, true),
                };

                f.write_str(s)?;

                if collapse_extensions {
                    let s = if extensions.ninth == Some(Accidental::Natural) {
                        extensions.ninth = None;
                        if extensions.eleventh == Some(Accidental::Natural) {
                            extensions.eleventh = None;

                            if extensions.thirteenth == Some(Accidental::Natural) {
                                extensions.thirteenth = None;

                                "13"
                            } else {
                                "11"
                            }
                        } else {
                            "9"
                        }
                    } else {
                        "7"
                    };

                    f.write_str(s)?;
                }

                if !matches!(
                    self.0.kind,
                    ChordKind::MajorSeventh(Alteration::SharpFive)
                        | ChordKind::Seventh(Alteration::SharpFive)
                        | ChordKind::MinorSeventh(Alteration::FlatFive)
                ) {
                    f.write_str(alteration.as_str())?;
                }

                if extensions != Extensions::none() {
                    f.write_char('(')?;

                    let mut has_predecessor = false;
                    if let Some(accidental) = extensions.ninth {
                        f.write_str(accidental.as_str())?;
                        f.write_char('9')?;
                        has_predecessor = true;
                    }

                    if let Some(accidental) = extensions.eleventh {
                        if has_predecessor {
                            f.write_char(',')?;
                        }
                        f.write_str(accidental.as_str())?;
                        f.write_str("11")?;
                        has_predecessor = true;
                    }

                    if let Some(accidental) = extensions.thirteenth {
                        if has_predecessor {
                            f.write_char(',')?;
                        }
                        f.write_str(accidental.as_str())?;
                        f.write_str("13")?;
                    }

                    f.write_char(')')?;
                }

                Ok(())
            }
        }

        S(*self)
    }

    pub fn sub(&self, scale: Scale) -> impl fmt::Display {
        struct S(ChordId, Scale);

        impl fmt::Display for S {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if let Some(note) = self.0.bass {
                    write!(f, "/{}", note.as_str(self.1))?
                }

                Ok(())
            }
        }

        S(*self, scale)
    }

    pub fn normal(&self) -> impl fmt::Display {
        struct S(ChordId);

        impl fmt::Display for S {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s = match self.0.kind {
                    ChordKind::Minor | ChordKind::MinorSixth | ChordKind::MinorMajorSeventh(_) => {
                        "m"
                    }
                    ChordKind::MinorSeventh(alteration) if alteration != Alteration::FlatFive => {
                        "m"
                    }
                    ChordKind::Augmented
                    | ChordKind::MajorSeventh(Alteration::SharpFive)
                    | ChordKind::Seventh(Alteration::SharpFive) => "+",

                    _ => return Ok(()),
                };
                f.write_str(s)
            }
        }

        S(*self)
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum ChordKind {
    Power,
    Major,
    Minor,
    Sus2,
    Sus4,
    Augmented,
    Diminished,
    Lydian,
    Phrygian,
    MinorSixth,
    MajorSixth,
    MinorMajorSeventh(Alteration),
    MajorSeventh(Alteration),
    MinorSeventh(Alteration),
    Seventh(Alteration),
    DiminishedSeventh, // 7bb5
}

impl ChordKind {
    const ALL: &'static [Self] = &[
        Self::Power,
        Self::Major,
        Self::Minor,
        Self::Sus2,
        Self::Sus4,
        Self::Augmented,
        Self::Diminished,
        Self::Lydian,
        Self::Phrygian,
        Self::MinorSixth,
        Self::MajorSixth,
        Self::MinorMajorSeventh(Alteration::None),
        Self::MajorSeventh(Alteration::None),
        Self::MinorSeventh(Alteration::None),
        Self::Seventh(Alteration::None),
        Self::DiminishedSeventh,
    ];
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum Alteration {
    None,
    FlatFive,
    NoThree,
    SharpFive,
    Sus2,
    Sus4,
}

impl Alteration {
    pub fn as_str(&self) -> &'static str {
        match self {
            Alteration::None => "",
            Alteration::FlatFive => "♭5",
            Alteration::NoThree => "no3",
            Alteration::SharpFive => "♯5",
            Alteration::Sus2 => "sus2",
            Alteration::Sus4 => "sus4",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Extensions {
    ninth: Option<Accidental>,
    eleventh: Option<Accidental>,
    thirteenth: Option<Accidental>,
}

impl Extensions {
    pub const fn none() -> Self {
        Self {
            ninth: None,
            eleventh: None,
            thirteenth: None,
        }
    }

    pub const fn ninth() -> Self {
        Self {
            ninth: Some(Accidental::Natural),
            eleventh: None,
            thirteenth: None,
        }
    }

    pub const fn flat_ninth() -> Self {
        Self {
            ninth: Some(Accidental::Flat),
            eleventh: None,
            thirteenth: None,
        }
    }

    pub const fn eleventh() -> Self {
        Self {
            eleventh: Some(Accidental::Natural),
            ninth: None,
            thirteenth: None,
        }
    }

    fn add(&mut self, base: Note, extension: Note) -> bool {
        if extension < base {
            return false;
        }

        match base.distance_to(extension) % 12 {
            1 => {
                if self.ninth.is_some() {
                    return false;
                } else {
                    self.ninth = Some(Accidental::Flat)
                }
            }

            2 => {
                if self.ninth.is_some() {
                    return false;
                } else {
                    self.ninth = Some(Accidental::Natural)
                }
            }

            3 => {
                if self.ninth.is_some() {
                    return false;
                } else {
                    self.ninth = Some(Accidental::Sharp)
                }
            }

            5 => {
                if self.eleventh.is_some() {
                    return false;
                } else {
                    self.eleventh = Some(Accidental::Natural)
                }
            }

            6 => {
                if self.eleventh.is_some() {
                    return false;
                } else {
                    self.eleventh = Some(Accidental::Sharp)
                }
            }

            8 => {
                if self.thirteenth.is_some() {
                    return false;
                } else {
                    self.thirteenth = Some(Accidental::Flat)
                }
            }

            9 => {
                if self.thirteenth.is_some() {
                    return false;
                } else {
                    self.thirteenth = Some(Accidental::Natural)
                }
            }

            _ => return false,
        }

        true
    }
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum Accidental {
    Flat,
    Natural,
    Sharp,
}

impl Accidental {
    pub fn as_str(&self) -> &'static str {
        match self {
            Accidental::Flat => "♭",
            Accidental::Natural => "",
            Accidental::Sharp => "♯",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use NoteName::*;

    macro_rules! map {
        ($($key:expr => $value:expr),*) => {{
            let mut map = BTreeMap::new();
            $(
                assert!(map.insert($key, $value).is_none());

            )*
            map
        }};
    }

    #[test]
    fn power() {
        let mut chord = chord![C4, G4];
        for _ in 0..chord.len() {
            assert_eq!(ChordKind::Power, chord.identify()[&C].kind);

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn major() {
        let mut chord = chord![C4, E4, G4];
        for _ in 0..chord.len() {
            assert_eq!(ChordKind::Major, chord.identify()[&C].kind);
            chord.invert_up().unwrap();
        }

        // add13 = MajorSixth
        for note in notes![
            Db5, // b9
            D5,  // 9
            Eb5, // #9
            F5,  // 11
            Gb5, // #11
            Ab5  // b13
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                ChordId {
                    bass: None,
                    kind: ChordKind::Major,
                    extensions,
                },
                chord![C4, E4, G4; note].identify()[&C]
            );
        }
    }

    #[test]
    fn minor() {
        let mut chord = chord![C4, Eb4, G4];
        for _ in 0..chord.len() {
            assert_eq!(ChordKind::Minor, chord.identify()[&C].kind);
            chord.invert_up().unwrap();
        }

        // add13 = MinorSixth
        for note in notes![
            D5,  // 9
            F5,  // 11
            Gb5  // #11
        ] {
            let mut extensions = Extensions::none();

            extensions.add(Note::C4, note);

            assert_eq!(
                ChordId {
                    bass: None,
                    kind: ChordKind::Minor,
                    extensions,
                },
                chord![C4, Eb4, G4; note].identify()[&C]
            );
        }
    }

    #[test]
    fn sus2() {
        assert_eq!(
            ChordId {
                bass: None,
                kind: ChordKind::Sus2,
                extensions: Extensions::none(),
            },
            chord![C4, D4, G4].identify()[&C]
        );

        for note in notes![
            Db5, // b9
            F5,  // 11
            Ab5, // b13
            A5   // 13
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                ChordId {
                    bass: None,
                    kind: ChordKind::Sus2,
                    extensions,
                },
                chord![C4, D4, G4; note].identify()[&C]
            );
        }
    }

    #[test]
    fn sus4() {
        assert_eq!(
            ChordId {
                bass: None,
                kind: ChordKind::Sus4,
                extensions: Extensions::none(),
            },
            chord![C4, F4, G4].identify()[&C]
        );

        for note in notes![
            Db5, // b9
            D5,  // 9
            Ab5, // b13
            A5   // 13
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                ChordId {
                    bass: None,
                    kind: ChordKind::Sus4,
                    extensions,
                },
                chord![C4, F4, G4; note].identify()[&C]
            );
        }
    }

    #[test]
    fn sus() {
        assert_eq!(
            map! {
                C => ChordId {
                    bass: Some(D),
                    kind: ChordKind::Sus2,
                    extensions: Extensions::none(),
                },
                G => ChordId {
                    bass: Some(D),
                    kind: ChordKind::Sus4,
                    extensions: Extensions::none(),
                }
            },
            chord![D4, G4, C5].identify()
        );
    }

    #[test]
    fn augmented() {
        assert_eq!(
            map! {
                C => ChordId {
                    bass: None,
                    kind: ChordKind::Augmented,
                    extensions: Extensions::none(),
                }
            },
            chord![C4, E4, Ab4].identify()
        );

        for note in notes![
            Db5, // b9
            D5,  // 9
            Eb5, // #9
            Gb5  // #11
        ] {
            let mut extensions = Extensions::none();

            extensions.add(Note::C4, note);

            assert_eq!(
                ChordId {
                    bass: None,
                    kind: ChordKind::Augmented,
                    extensions,
                },
                chord![C4, E4, Ab4; note].identify()[&C]
            );
        }
    }

    #[test]
    fn diminished() {
        let mut chord = chord![C4, Eb4, Gb4];
        for _ in 0..chord.len() {
            assert_eq!(ChordKind::Diminished, chord.identify()[&C].kind);

            chord.invert_up().unwrap();
        }

        for note in notes![D5, F5, Ab5] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                ChordId {
                    bass: None,
                    kind: ChordKind::Diminished,
                    extensions,
                },
                chord![C4, Eb4, Gb4; note].identify()[&C]
            );
        }
    }

    #[test]
    fn lydian() {
        let mut chord = chord![C4, Gb4, G4];
        for _ in 0..chord.len() {
            assert_eq!(ChordKind::Lydian, chord.identify()[&C].kind);

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn phrygian() {
        let mut chord = chord![C4, Db4, G4];
        for _ in 0..chord.len() {
            assert_eq!(ChordKind::Phrygian, chord.identify()[&C].kind);

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn major_sixth() {
        assert_eq!(
            ChordId {
                bass: None,
                kind: ChordKind::MajorSixth,
                extensions: Extensions::none(),
            },
            chord![C4, E4, G4, A4].identify()[&C]
        );

        for note in notes![
            D5 // 9
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                ChordId {
                    bass: None,
                    kind: ChordKind::MajorSixth,
                    extensions,
                },
                chord![C4, E4, G4, A4; note].identify()[&C]
            );
        }
    }

    #[test]
    fn minor_sixth() {
        assert_eq!(
            ChordId {
                bass: None,
                kind: ChordKind::MinorSixth,
                extensions: Extensions::none(),
            },
            chord![C4, Eb4, G4, A4].identify()[&C]
        );

        for note in notes![
            D5 // 9
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                ChordId {
                    bass: None,
                    kind: ChordKind::MinorSixth,
                    extensions,
                },
                chord![C4, Eb4, G4, A4; note].identify()[&C]
            );
        }
    }

    #[test]
    fn minor_major_seventh() {
        let mut chord = chord![C4, Eb4, G4, B4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MinorMajorSeventh(Alteration::None),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }

        for note in notes![
            D5, // 9
            F5, // 11
            A5  // 13
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                ChordId {
                    bass: None,
                    kind: ChordKind::MinorMajorSeventh(Alteration::None),
                    extensions
                },
                chord![C4, Eb4, G4, B4; note].identify()[&C]
            );
        }
    }

    #[test]
    fn minor_major_seventh_no_five() {
        let mut chord = chord![C4, Eb4, B4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MinorMajorSeventh(Alteration::None),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn minor_major_seventh_sharp_five() {
        let mut chord = chord![C4, Eb4, Ab4, B4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MinorMajorSeventh(Alteration::SharpFive),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn major_seventh() {
        let mut chord = chord![C4, E4, G4, B4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MajorSeventh(Alteration::None),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }

        for note in notes![
            D5,  // 9
            Gb5, // #11
            A5   // 13
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                map! {
                    C => ChordId {
                        bass: None,
                        kind: ChordKind::MajorSeventh(Alteration::None),
                        extensions,
                    }
                },
                chord![C4, E4, G4, B4; note].identify()
            );
        }
    }

    #[test]
    fn major_seventh_sus2() {
        let mut chord = chord![C4, D4, G4, B4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MajorSeventh(Alteration::Sus2),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn major_seventh_sus4() {
        let mut chord = chord![C4, F4, G4, B4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MajorSeventh(Alteration::Sus4),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn major_seventh_flat_five() {
        let mut chord = chord![C4, E4, Gb4, B4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MajorSeventh(Alteration::FlatFive),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn major_seventh_sharp_five() {
        let mut chord = chord![C4, E4, Ab4, B4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MajorSeventh(Alteration::SharpFive),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }

        for note in notes![
            D5,  // 9
            Gb5  // #11
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                ChordId {
                    bass: None,
                    kind: ChordKind::MajorSeventh(Alteration::SharpFive),
                    extensions
                },
                chord![C4, E4, Ab4, B4; note].identify()[&C]
            );
        }
    }

    #[test]
    fn major_seventh_no_three() {
        let mut chord = chord![C4, G4, B4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MajorSeventh(Alteration::NoThree),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn major_seventh_no_five() {
        let mut chord = chord![C4, E4, B4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MajorSeventh(Alteration::None),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn minor_seventh() {
        let mut chord = chord![C4, Eb4, G4, Bb4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MinorSeventh(Alteration::None),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }

        for note in notes![
            D5, // 9
            F5, // 11
            A5  // 13
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                map! {
                    C => ChordId {
                        bass: None,
                        kind: ChordKind::MinorSeventh(Alteration::None),
                        extensions,
                    }
                },
                chord![C4, Eb4, G4, Bb4; note].identify()
            );
        }
    }

    #[test]
    fn minor_seventh_flat_five() {
        let mut chord = chord![C4, Eb4, Gb4, Bb4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MinorSeventh(Alteration::FlatFive),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }

        for note in notes![
            D5,  // 9
            F5,  // 11
            Ab5  // b13
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                map! {
                    C => ChordId {
                        bass: None,
                        kind: ChordKind::MinorSeventh(Alteration::FlatFive),
                        extensions,
                    }
                },
                chord![C4, Eb4, Gb4, Bb4; note].identify()
            );
        }
    }

    #[test]
    fn minor_seventh_sharp_five() {
        let mut chord = chord![C4, Eb4, Ab4, Bb4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MinorSeventh(Alteration::SharpFive),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn minor_seventh_no_five() {
        let mut chord = chord![C4, Eb4, Bb4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::MinorSeventh(Alteration::None),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn seventh() {
        let mut chord = chord![C4, E4, G4, Bb4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::Seventh(Alteration::None),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }

        for note in notes![
            Db5, // b9
            D5,  // 9
            // Eb5, // #9 // FIXME
            Gb5, // #11
            Ab5, // b13
            A5   // 13
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                ChordId {
                    bass: None,
                    kind: ChordKind::Seventh(Alteration::None),
                    extensions
                },
                chord![C4, E4, G4, Bb4; note].identify()[&C]
            );
        }
    }

    #[test]
    fn seventh_sus2() {
        let mut chord = chord![C4, D4, G4, Bb4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::Seventh(Alteration::Sus2),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn seventh_sus4() {
        let mut chord = chord![C4, F4, G4, Bb4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::Seventh(Alteration::Sus4),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }

        for note in notes![
            Db5, // b9
            D5,  // 9
            // Eb5, // #9 FIXME
            // E5, // b11 FIXME
            Ab5, // b13
            A5   // 13
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                ChordId {
                    bass: None,
                    kind: ChordKind::Seventh(Alteration::Sus4),
                    extensions
                },
                chord![C4, F4, G4, Bb4; note].identify()[&C]
            );
        }
    }

    #[test]
    fn seventh_flat_five() {
        let mut chord = chord![C4, E4, Gb4, Bb4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::Seventh(Alteration::FlatFive),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn seventh_sharp_five() {
        let mut chord = chord![C4, E4, Ab4, Bb4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::Seventh(Alteration::SharpFive),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }

        for note in notes![
            Db5, // b9
            D5,  // 9
            // Eb5, // #9 FIXME
            Gb5, // #11
            A5   // 13
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                ChordId {
                    bass: None,
                    kind: ChordKind::Seventh(Alteration::SharpFive),
                    extensions
                },
                chord![C4, E4, Ab4, Bb4; note].identify()[&C]
            );
        }
    }

    #[test]
    fn seventh_no_five() {
        let mut chord = chord![C4, E4, Bb4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::Seventh(Alteration::None),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn seventh_no_three() {
        let mut chord = chord![C4, G4, Bb4];
        for _ in 0..chord.len() {
            assert_eq!(
                ChordKind::Seventh(Alteration::NoThree),
                chord.identify()[&C].kind
            );

            chord.invert_up().unwrap();
        }
    }

    #[test]
    fn diminished_seventh() {
        assert_eq!(
            map! {
                C => ChordId {
                    bass: None,
                    kind: ChordKind::DiminishedSeventh,
                    extensions: Extensions::none(),
                }
            },
            chord![C4, Eb4, Gb4, A4].identify()
        );

        for note in notes![
            D5, // 9
            F5, // 11
            Ab5  // b13
                // B5, // 7 TODO
        ] {
            let mut extensions = Extensions::none();
            extensions.add(Note::C4, note);

            assert_eq!(
                map! {
                    C => ChordId {
                        bass: None,
                        kind: ChordKind::DiminishedSeventh,
                        extensions,
                    }
                },
                chord![C4, Eb4, Gb4, A4; note].identify()
            );
        }
    }

    #[test]
    fn inverted_ninth() {
        assert_eq!(
            ChordId {
                bass: Some(E),
                kind: ChordKind::Seventh(Alteration::None),
                extensions: Extensions::ninth(),
            },
            chord![E4, G4, Bb4, C5, D5].identify()[&C]
        );
    }

    #[test]
    fn minor_seventh_quartal_voicing() {
        assert_eq!(
            ChordId {
                bass: None,
                kind: ChordKind::MinorSeventh(Alteration::None),
                extensions: Extensions::eleventh(),
            },
            chord![D4, G4, C5, F5, A5].identify()[&D]
        );
    }

    #[test]
    fn regression() {
        assert_eq!(
            map! {
                E => ChordId {
                    bass: None,
                    kind: ChordKind::MinorSeventh(Alteration::None),
                    extensions: Extensions::ninth(),
                }
            },
            chord![E4, B4, D5, Gb5, G5].identify()
        );
    }

    #[test]
    fn regression_inversion() {
        assert_eq!(
            map! {
                Eb => ChordId {
                    bass: Some(NoteName::G),
                    kind: ChordKind::Major,
                    extensions: Extensions::none(),
                }
            },
            chord![G3, G4, Bb4, Eb5].identify()
        );
    }

    #[test]
    fn d_over_e() {
        assert_eq!(
            ChordId {
                bass: None,
                kind: ChordKind::Seventh(Alteration::Sus4),
                extensions: Extensions::ninth(),
            },
            chord![E4, D5, Gb5, A5].identify()[&E]
        );
    }

    #[test]
    fn add_nine_no_five() {
        assert_eq!(
            map! {
                D => ChordId {
                    bass: Some(Gb),
                    kind: ChordKind::Major,
                    extensions: Extensions::ninth(),
                }
            },
            chord![Gb4, D5, E5].identify()
        );
    }
}
