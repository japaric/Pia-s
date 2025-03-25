use web::MIDIAccess;

use crate::html::Form;

pub mod device;

pub(super) fn initialize(form: &Form, midi_access: MIDIAccess) {
    let fieldset = form.fieldset(&"MIDI Input".into());

    device::initialize(&fieldset, midi_access);
}
