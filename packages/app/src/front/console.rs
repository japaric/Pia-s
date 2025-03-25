use alloc::string::ToString;
use music::{Interval, NoteName, Notes, Scale};
use spur::{Message, Publish as _, React};
use web::{HtmlDivElement, Node};

use crate::broker::Broker;
use crate::class::Class;
use crate::messages::ActiveNotesChanged;
use crate::{consts, html};

pub(super) fn initialize(parent: &Node) {
    let container = &html::div(parent, Class::Console);
    let notes = html::div(container, Class::Padded);
    let intervals = html::div(container, Class::Padded);

    Broker::publish(Initialize {
        state: State {
            intervals,
            notes,
            scale: Scale::new(
                NoteName::CIRCLE_OF_FIFTHS[consts::INITIAL_SCALE_TONIC_INDEX as usize],
            ),
        },
    });
}

pub struct Console {
    state: Option<State>,
}

impl Console {
    pub const fn new() -> Self {
        Self { state: None }
    }
}

impl React<Initialize> for Console {
    fn react(&mut self, Initialize { state }: Initialize) {
        self.state = Some(state);
    }
}

#[derive(Message)]
pub struct Initialize {
    state: State,
}

impl React<ActiveNotesChanged> for Console {
    fn react(&mut self, ActiveNotesChanged { held, sustained }: ActiveNotesChanged) {
        let Some(State {
            notes,
            intervals,
            scale,
            ..
        }) = &mut self.state
        else {
            return;
        };

        notes.replace_children0();
        intervals.replace_children0();

        let all = held.union(&sustained);

        if all.len() < 2 {
            return;
        }

        display_notes(notes, scale, &all);
        display_intervals(intervals, &all);
    }
}

fn display_notes(notes: &mut HtmlDivElement, scale: &mut Scale, all: &Notes) {
    let mut is_first = true;
    for note in all.iter() {
        if !is_first {
            html::span(notes, " ");
        }

        html::span(notes, &note.display(*scale).to_string());

        is_first = false;
    }
}

fn display_intervals(intervals: &mut HtmlDivElement, all: &Notes) {
    let mut notes = all.iter();
    let mut last = notes.next().unwrap();
    let mut is_first = true;
    for note in notes {
        let half_steps = last.distance_to(note);
        let interval = Interval::from_i8_lossy(half_steps);

        if !is_first {
            html::span(intervals, " ");
        }

        html::span(intervals, interval.as_str());

        last = note;
        is_first = false;
    }
}

struct State {
    scale: Scale,
    notes: HtmlDivElement,
    intervals: HtmlDivElement,
}
