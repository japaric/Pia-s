use web::{MIDIAccess, Node};

pub mod canvas;
pub mod console;
pub mod settings;
pub mod tonnetz;

pub fn initialize(parent: &Node, midi_access: MIDIAccess) {
    settings::initialize(parent, midi_access);
    tonnetz::initialize(parent);
    canvas::initialize(parent);
    console::initialize(parent);
}
