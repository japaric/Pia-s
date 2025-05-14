#![cfg_attr(not(test), no_std)]

#[cfg(test)]
#[macro_use]
mod macros;
mod chord;
pub mod chord_id;
mod degree;
mod interval;
mod major_scale;
mod note;
mod note_name;
mod note_names;
pub mod notes;
mod scale;
mod scale_type;

pub use chord::Chord;
pub use degree::Degree;
pub use interval::Interval;
pub use major_scale::MajorScale;
pub use note::Note;
pub use note_name::NoteName;
pub use note_names::NoteNames;
pub use notes::Notes;
pub use scale::Scale;
pub use scale_type::ScaleType;

#[cfg_attr(test, derive(Debug))]
pub enum Error {
    NoteOutOfRange,
    CannotInvert,
}
