use js::{Downcast as _, Upcast as _};
use music::Note;
use spur::{Message, Publish as _, React};
use web::{HtmlSelectElement, MIDIAccess, MIDIMessageEvent, Node};

use crate::broker::Broker;
use crate::messages::{HoldPedalPressed, HoldPedalReleased, NoteOff, NoteOn};
use crate::{consts, html};

pub(super) fn initialize(parent: &Node, midi_access: MIDIAccess) {
    let select = html::select(parent, &"midi-input-device".into());
    select.set_required(true);

    html::option(&select, None, &"(disconnected)".into());

    for (key, midi_input) in midi_access.inputs().entries() {
        let name = midi_input.name();
        html::option(&select, Some(&key), &name);
    }

    select.set_onchange(|event| {
        Broker::publish(SelectChanged {
            select: event.target().upcast().upcast().downcast(),
        })
    });

    Broker::publish(Initialize { midi_access });
}

pub struct MidiInputDeviceSelect {
    state: Option<State>,
}

impl React<Initialize> for MidiInputDeviceSelect {
    fn react(&mut self, Initialize { midi_access }: Initialize) {
        self.state = Some(State {
            midi_access,
            last_selected: None,
        })
    }
}

impl React<SelectChanged> for MidiInputDeviceSelect {
    fn react(&mut self, message: SelectChanged) {
        let Some(state) = &mut self.state else { return };

        let input_id = message.select.value();
        let inputs = state.midi_access.inputs();

        let Some(new_input) = inputs.get(&input_id) else {
            return;
        };

        if let Some(old_input_id) = &state.last_selected {
            if let Some(old_input) = inputs.get(old_input_id) {
                old_input.set_onmidimessage(None);
            }
        }

        new_input.set_onmidimessage(Some(onmidimessage));

        state.last_selected = Some(input_id);
    }
}

impl MidiInputDeviceSelect {
    pub const fn new() -> Self {
        Self { state: None }
    }
}

#[derive(Message)]
pub struct Initialize {
    midi_access: MIDIAccess,
}

#[derive(Message)]
pub struct SelectChanged {
    select: HtmlSelectElement,
}

struct State {
    midi_access: MIDIAccess,
    last_selected: Option<js::String>,
}

fn onmidimessage(message: MIDIMessageEvent) {
    let data = message.data();
    // only interested in NoteOn, NoteOff and ControlChange messages which are 3-byte long
    if data.length() != 3 {
        return;
    }

    let mut buf = [0; 3];
    data.copy_to_slice(&mut buf);

    let Some(message) = parse(buf) else { return };

    match message {
        MidiMessage::NoteOn(note) => Broker::publish(NoteOn(note)),
        MidiMessage::NoteOff(note) => Broker::publish(NoteOff(note)),
        MidiMessage::SustainPedal(on) => {
            if on {
                Broker::publish(HoldPedalPressed)
            } else {
                Broker::publish(HoldPedalReleased)
            }
        }
    }
}

fn parse(buf: [u8; 3]) -> Option<MidiMessage> {
    const CMD_CONTROL_CHANGE: u8 = 0b1011;
    const CMD_NOTE_OFF: u8 = 0b1000;
    const CMD_NOTE_ON: u8 = 0b1001;
    const CONTROL_HOLD_PEDAL: u8 = 64;
    const HOLD_PEDAL_THRESHOLD: u8 = 64;
    const MASK_CHANNEL: u8 = 0b0000_1111;
    const MASK_COMMAND: u8 = 0b1111_0000;

    let status = buf[0];
    let command = (status & MASK_COMMAND) >> 4;
    let channel = status & MASK_CHANNEL;

    if channel != consts::MIDI_INPUT_CHANNEL {
        return None;
    }

    let note = buf[1];
    let velocity = buf[2];
    if note >= 0x80 || velocity >= 0x80 {
        return None;
    }

    let message = if command == CMD_NOTE_ON {
        let note = Note::from_u8_lossy(note);
        MidiMessage::NoteOn(note)
    } else if command == CMD_NOTE_OFF {
        let note = Note::from_u8_lossy(note);
        MidiMessage::NoteOff(note)
    } else if command == CMD_CONTROL_CHANGE {
        let ctrl_number = note;
        let ctrl_value = velocity;
        if ctrl_number == CONTROL_HOLD_PEDAL {
            MidiMessage::SustainPedal(ctrl_value >= HOLD_PEDAL_THRESHOLD)
        } else {
            return None;
        }
    } else {
        return None;
    };

    Some(message)
}

enum MidiMessage {
    NoteOn(Note),
    NoteOff(Note),
    SustainPedal(bool),
}
