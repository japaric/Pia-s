use web::{MIDIAccess, Node};

use crate::class::Class;
use crate::html;

pub mod midi_input;
pub mod scale;

pub(super) fn initialize(parent: &Node, midi_access: MIDIAccess) {
    let form = html::form(parent, Class::ColumnContainer);
    midi_input::initialize(&form, midi_access);
    scale::initialize(&form);
}
