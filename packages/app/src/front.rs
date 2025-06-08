use web::{MIDIAccess, Node};

use crate::{class::Class, html};

pub mod canvas;
pub mod cof;
pub mod console;
pub mod settings;
pub mod tonnetz;

pub fn initialize(parent: &Node, midi_access: MIDIAccess) {
    settings::initialize(parent, midi_access);
    canvas::initialize(parent);
    let row = html::div(parent, Class::ColumnContainer);
    tonnetz::initialize(&row);
    cof::initialize(&row);
    console::initialize(parent);
}
