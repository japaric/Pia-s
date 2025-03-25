use web::{MIDIAccess, Node};

pub mod canvas;
pub mod console;
pub mod settings;

pub fn initialize(parent: &Node, midi_access: MIDIAccess) {
    settings::initialize(parent, midi_access);
    canvas::initialize(parent);
    console::initialize(parent);
}
