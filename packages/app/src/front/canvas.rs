use music::{NoteName, Notes, ScaleType};
use piano::Piano;
use spur::{Message, Publish as _, React};
use web::Node;

use crate::broker::Broker;
use crate::class::Class;
use crate::messages::{ActiveNotesChanged, NewScaleTonicSelected, NewScaleTypeSelected};
use crate::{consts, svg};

mod piano;

pub(super) fn initialize(parent: &Node) {
    let piano = Piano::new(&svg::svg(parent, Class::Piano));

    Broker::publish(Initialize { piano })
}

pub struct Canvas {
    state: Option<State>,
}

impl Canvas {
    pub const fn new() -> Self {
        Self { state: None }
    }
}

#[derive(Message)]
pub struct Initialize {
    piano: Piano,
}

struct State {
    last_held: Notes,
    last_sustained: Notes,
    piano: Piano,
    scale_tonic: NoteName,
    scale_type: ScaleType,
}

impl React<Initialize> for Canvas {
    fn react(&mut self, Initialize { piano }: Initialize) {
        self.state = Some(State {
            piano,
            last_held: Notes::empty(),
            last_sustained: Notes::empty(),
            scale_tonic: NoteName::CIRCLE_OF_FIFTHS[consts::INITIAL_SCALE_TONIC_INDEX as usize],
            scale_type: ScaleType::ALL[consts::INITIAL_SCALE_TYPE_INDEX as usize],
        });
    }
}

impl React<NewScaleTonicSelected> for Canvas {
    fn react(&mut self, NewScaleTonicSelected(index): NewScaleTonicSelected) {
        let Some(State {
            piano,
            scale_tonic,
            scale_type,
            ..
        }) = &mut self.state
        else {
            return;
        };

        *scale_tonic = NoteName::CIRCLE_OF_FIFTHS[index];
        piano.set_scale(*scale_tonic, *scale_type);
    }
}

impl React<NewScaleTypeSelected> for Canvas {
    fn react(&mut self, NewScaleTypeSelected(index): NewScaleTypeSelected) {
        let Some(State {
            piano,
            scale_type,
            scale_tonic,
            ..
        }) = &mut self.state
        else {
            return;
        };

        *scale_type = ScaleType::ALL[index];
        piano.set_scale(*scale_tonic, *scale_type);
    }
}

impl React<ActiveNotesChanged> for Canvas {
    fn react(
        &mut self,
        ActiveNotesChanged {
            held: current_held,
            sustained: current_sustained,
        }: ActiveNotesChanged,
    ) {
        let Some(State {
            last_held,
            last_sustained,
            piano,
            ..
        }) = &mut self.state
        else {
            return;
        };

        for note in current_held.difference(last_held) {
            piano.pressed(note);
        }

        for note in last_held.difference(&current_held) {
            piano.released(note);
        }

        for note in current_sustained.difference(last_sustained) {
            piano.sustain_on(note);
        }

        for note in last_sustained.difference(&current_sustained) {
            piano.sustain_off(note);
        }

        *last_held = current_held;
        *last_sustained = current_sustained;
    }
}
