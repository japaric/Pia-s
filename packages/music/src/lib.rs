#![cfg_attr(not(test), no_std)]

mod degree;
mod interval;
mod note;
mod note_name;
mod notes;
mod scale;
mod scale_type;

pub use degree::Degree;
pub use interval::Interval;
pub use note::Note;
pub use note_name::NoteName;
pub use notes::Notes;
pub use scale::Scale;
pub use scale_type::ScaleType;
