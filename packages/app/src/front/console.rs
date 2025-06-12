use alloc::collections::btree_map::BTreeMap;
use alloc::format;
use alloc::string::ToString;
use music::{Chord, Interval, Note, NoteName, NoteNames, Notes, Scale, ScaleType};
use spur::{Message, Publish as _, React};
use web::{HtmlDivElement, Node};

use crate::broker::Broker;
use crate::class::Class;
use crate::messages::{
    ActiveHarmonyChanged, ActiveNotesChanged, NewScaleTonicSelected, NewScaleTypeSelected,
};
use crate::{consts, html};

pub(super) fn initialize(parent: &Node) {
    let container = &html::div(parent, Class::Console);
    let chord_id = html::div(container, Class::Padded);
    let notes = html::div(container, Class::Padded);
    let intervals = html::div(container, Class::Padded);

    Broker::publish(Initialize {
        state: State {
            held_and_sustained: Notes::empty(),
            intervals,
            notes,
            chord_id,
            scale: Scale::major(
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
        let Some(state) = &mut self.state else {
            return;
        };

        state.held_and_sustained = held.union(&sustained);
        state.refresh();
    }
}

impl React<NewScaleTonicSelected> for Console {
    fn react(&mut self, NewScaleTonicSelected(index): NewScaleTonicSelected) {
        if let Some(state) = &mut self.state {
            let tonic = NoteName::CIRCLE_OF_FIFTHS[index];
            state.scale.tonic = tonic;
            state.refresh();
        }
    }
}

impl React<NewScaleTypeSelected> for Console {
    fn react(&mut self, NewScaleTypeSelected(index): NewScaleTypeSelected) {
        if let Some(state) = &mut self.state {
            let ty = ScaleType::ALL[index];
            state.scale.ty = ty;
            state.refresh();
        }
    }
}

fn display_notes(notes: &HtmlDivElement, scale: Scale, all: &Notes) {
    let mut is_first = true;
    for note in all.iter() {
        if !is_first {
            html::span(notes, " ");
        }

        html::span(notes, &note.display(scale).to_string());

        is_first = false;
    }
}

fn display_intervals(intervals: &HtmlDivElement, all: &Notes) {
    let mut notes = all.iter();
    let mut last = notes.next().unwrap();
    let mut is_first = true;
    let lil_warning = js::String::from(Class::LilWarning.as_str());
    for note in notes {
        let half_steps = last.distance_to(note);
        let interval = Interval::from_u8_lossy(half_steps.unsigned_abs());

        if !is_first {
            html::span(intervals, " ");
        }

        let span = html::span(intervals, interval.as_str());
        if half_steps < 24 && below_low_interval_limit(last, interval) {
            span.add_class(&lil_warning);
        }

        last = note;
        is_first = false;
    }
}

fn below_low_interval_limit(lower: Note, interval: Interval) -> bool {
    use Interval::*;

    match interval {
        // unlimited
        P0 | P8 | P15 => false,

        m2 => lower < Note::E3,
        M2 => lower < Note::Eb3,
        m3 => lower < Note::C3,
        M3 => lower < Note::Bb2,
        P4 => lower < Note::Bb2,
        TT => lower < Note::Bb2,
        P5 => lower < Note::Bb1,
        m6 => lower < Note::G2,
        M6 => lower < Note::F2,
        m7 => lower < Note::F2,
        M7 => lower < Note::F2,
        m9 => lower < Note::E2,
        M9 => lower < Note::Eb2,
        m10 => lower < Note::C2,
        M10 => lower < Note::Bb1,

        // no data
        P11 | A11 | P12 | m13 | M13 | m14 | M14 => false,
    }
}

fn display_chord_id(chord_id: &HtmlDivElement, all: &Notes, scale: Scale) {
    let Ok(chord) = Chord::try_from(all.clone()) else {
        return;
    };

    let mut note_names = NoteNames::empty();
    for note in chord.notes() {
        note_names.insert(note.name());
    }

    let mut is_first = true;
    let mut tonics = BTreeMap::new();
    for tonic in note_names {
        if let Some(id) = chord.identify_with_tonic(tonic) {
            tonics.insert(tonic, id.kind.is_minor());

            if !is_first {
                html::span(chord_id, " or ");
            }

            let span = html::span(chord_id, &format!("{}{}", tonic.as_str(scale), id.normal()));
            html::sup(&span, &id.sup().to_string());
            html::sub(&span, &id.sub(scale).to_string());

            is_first = false;
        }
    }

    Broker::publish(ActiveHarmonyChanged { tonics });

    let found_no_id = is_first;
    if found_no_id {
        let intentionally_blank = "　";

        let span = html::span(chord_id, intentionally_blank);
        html::sup(&span, intentionally_blank);
        html::sub(&span, intentionally_blank);
    }
}

struct State {
    held_and_sustained: Notes,
    scale: Scale,
    notes: HtmlDivElement,
    intervals: HtmlDivElement,
    chord_id: HtmlDivElement,
}

impl State {
    fn refresh(&self) {
        let Self {
            scale,
            notes,
            intervals,
            chord_id,
            held_and_sustained,
        } = self;

        notes.replace_children0();
        intervals.replace_children0();
        chord_id.replace_children0();

        if held_and_sustained.len() < 2 {
            Broker::publish(ActiveHarmonyChanged {
                tonics: BTreeMap::new(),
            });

            return;
        }

        display_notes(notes, *scale, held_and_sustained);
        display_intervals(intervals, held_and_sustained);
        display_chord_id(chord_id, held_and_sustained, *scale);
    }
}
