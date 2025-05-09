use alloc::collections::btree_map::BTreeMap;
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
    last_overtone: Notes,
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
            last_overtone: Notes::empty(),
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
            last_overtone,
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

        let mut overtone_power = BTreeMap::new();
        for note in current_held.union(&current_sustained) {
            for (half_steps, den) in OVERTONES.iter().copied().zip(2..) {
                if let Ok(note) = note.step(half_steps) {
                    if current_held.contains(note) || current_sustained.contains(note) {
                        continue;
                    }

                    *overtone_power.entry(note).or_default() += 1. / den as f64;
                }
            }
        }

        for (note, power) in &overtone_power {
            piano.overtone_on(*note, *power);
        }

        for note in last_overtone.clone() {
            if !overtone_power.contains_key(&note) {
                piano.overtone_off(note);
            }
        }

        *last_overtone = overtone_power.keys().copied().collect();
        *last_held = current_held;
        *last_sustained = current_sustained;
    }
}

const OVERTONES: &[i8] = &[
    12, // P8
    19, // P8 + P5
    24, // 2*P8
    28, // 2*P8+M3
    31, // 2*P8+P5
    34, // 2*P8+m7
    36, // 3*P8
    38, // 3*P8+M2
    40, // 3*P8+M3
];
