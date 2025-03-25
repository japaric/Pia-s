use music::Note;

pub const INITIAL_SCALE_TONIC_INDEX: u8 = 6; // NoteName::C
pub const INITIAL_SCALE_TYPE_INDEX: u8 = 0; // ScaleType::Major

pub const MIN_KEY: Note = Note::A0;
pub const MAX_KEY: Note = Note::C8;

pub const MIDI_INPUT_CHANNEL: u8 = 0; // Ch1
