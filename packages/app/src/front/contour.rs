use alloc::collections::btree_map::BTreeMap;
use alloc::format;
use alloc::vec::Vec;
use music::{MajorScale, Note, NoteName, Notes};
use spur::{Message, Publish, React};
use web::{Node, Performance, SVGAnimateElement, SVGRectElement, SVGSVGElement};

use crate::broker::Broker;
use crate::class::Class;
use crate::messages::{NoteOff, NoteOn};
use crate::{consts, svg};

pub(super) fn initialize(parent: &Node) {
    let canvas = Canvas::new(&svg::svg(parent, Class::Contour, false));

    Broker::publish(Initialize { canvas });
}

pub struct Contour {
    state: Option<State>,
}

impl Contour {
    pub const fn new() -> Self {
        Self { state: None }
    }
}

#[derive(Message)]
pub struct Initialize {
    canvas: Canvas,
}

impl React<Initialize> for Contour {
    fn react(&mut self, Initialize { canvas }: Initialize) {
        self.state = Some(State::new(canvas));
    }
}

struct State {
    canvas: Canvas,
    begin_zero: f64,
    held: Notes,
}

impl State {
    fn new(canvas: Canvas) -> Self {
        Self {
            canvas,
            begin_zero: Performance.now(),
            held: Notes::empty(),
        }
    }
}

impl React<NoteOn> for Contour {
    fn react(&mut self, NoteOn(note, timestamp): NoteOn) {
        let Some(state) = &mut self.state else {
            return;
        };

        let begin_timestamp = timestamp - state.begin_zero;
        state.canvas.gc(begin_timestamp);

        state.canvas.on(note, begin_timestamp);

        state.held.insert(note);
    }
}

impl React<NoteOff> for Contour {
    fn react(&mut self, NoteOff(note, timestamp): NoteOff) {
        let Some(state) = &mut self.state else {
            return;
        };

        let begin_timestamp = timestamp - state.begin_zero;
        state.canvas.gc(begin_timestamp);

        state.canvas.off(note, begin_timestamp);

        state.held.remove(note);
    }
}

struct Canvas {
    scale: MajorScale,
    root: SVGSVGElement,
    active: BTreeMap<Note, AnimatedLine>,
    notes: BTreeMap<i64, Vec<SVGRectElement>>,
}

struct AnimatedLine {
    translate: SVGAnimateElement,
    line: SVGRectElement,
    start: f64,
}

const DUR: f64 = 8.;
const SEMITONE_GAP: u32 = 10;
const MIN_NOTE: Note = Note::A0;
const MAX_NOTE: Note = Note::C8;

impl Canvas {
    fn new(parent: &SVGSVGElement) -> Self {
        let scale =
            MajorScale::new(NoteName::CIRCLE_OF_FIFTHS[consts::INITIAL_SCALE_TONIC_INDEX as usize]);

        parent.set_height(&js::String::from("300px"));
        parent.set_width(&js::String::from("100%"));
        parent.set_view_box(&js::String::from("0 340 800 300"));

        for octave in 0..9 {
            let Ok(p0) = scale.tonic().with_octave(octave) else {
                continue;
            };

            if p0 < MIN_NOTE || p0 > MAX_NOTE {
                continue;
            }

            let y = p0.distance_to(MAX_NOTE) as u32 * SEMITONE_GAP + SEMITONE_GAP / 2;

            svg::rect(
                parent,
                Class::ContourGridMajor,
                &js::Integer::from(0),
                &js::Integer::from(y),
                &js::String::from("100%"),
                &js::Integer::from(1),
            );

            let Ok(p5) = p0.step(7) else { continue };

            let y = p5.distance_to(MAX_NOTE) as u32 * SEMITONE_GAP + SEMITONE_GAP / 2;

            svg::rect(
                parent,
                Class::ContourGridMinor,
                &js::Integer::from(0),
                &js::Integer::from(y),
                &js::String::from("100%"),
                &js::Integer::from(1),
            );
        }

        Self {
            root: parent.clone(),
            active: BTreeMap::new(),
            notes: BTreeMap::new(),
            scale,
        }
    }

    fn on(&mut self, note: Note, now_ms: f64) {
        let y = note.distance_to(MAX_NOTE) as u32 * SEMITONE_GAP + SEMITONE_GAP / 2;

        let line = svg::rect(
            &self.root,
            Class::ContourLine,
            &js::String::from("100%"),
            &js::Integer::from(y),
            &js::String::from("0"),
            &js::Integer::from(1),
        );
        line.set_rx(&js::Integer::from(3));
        line.set_ry(&js::Integer::from(3));
        let degree = self.scale.name2degree(note.name());
        line.add_class(&js::String::from(degree.as_str()));

        let now = now_ms / 1000.;
        let now_s = format!("{}s", now).as_str().into();
        let translate = svg::animate(
            &line,
            &"x".into(),
            &now_s,
            &"8s".into(),
            &"100%".into(),
            &"0%".into(),
        );
        translate.set_fill(&"freeze".into());

        let grow = svg::animate(
            &line,
            &"width".into(),
            &now_s,
            &"8s".into(),
            &"0".into(),
            &"100%".into(),
        );
        grow.set_fill(&"freeze".into());

        self.active.insert(
            note,
            AnimatedLine {
                line,
                start: now,
                translate,
            },
        );
    }

    fn off(&mut self, note: Note, now_ms: f64) {
        let Some(AnimatedLine {
            translate,
            line,
            start,
        }) = self.active.remove(&note)
        else {
            return;
        };

        let now = now_ms / 1000.;
        let now_s = format!("{}s", now).as_str().into();

        let freeze_s = "freeze".into();
        line.replace_children0();
        if now > start + DUR {
            // shrink now
            svg::animate(
                &line,
                &"width".into(),
                &now_s,
                &"8s".into(),
                &"100%".into(),
                &"0%".into(),
            )
            .set_fill(&freeze_s);

            let deadline = (1000. * (now + DUR)) as i64 + 1;
            self.notes.entry(deadline).or_default().push(line);
        } else {
            let pct = 100. * (now - start) / DUR;

            let dur = DUR * pct / 100.;
            let dur_s = format!("{}s", dur).as_str().into();
            let then = start + DUR;
            let then_s = format!("{}s", then).as_str().into();
            let pct_s: js::String = format!("{pct}%").as_str().into();
            line.set_width(&pct_s);

            // shrink then
            svg::animate(
                &line,
                &"width".into(),
                &then_s,
                &dur_s,
                &pct_s,
                &"0%".into(),
            )
            .set_fill(&freeze_s);

            line.append_child(&translate);

            let deadline = (1000. * (then + dur)) as i64 + 1;
            self.notes.entry(deadline).or_default().push(line);
        };
    }

    fn gc(&mut self, now: f64) {
        let now = now as i64;
        self.notes.retain(|deadline, lines| {
            if *deadline > now {
                true
            } else {
                for line in lines {
                    self.root.remove_child(line);
                }
                false
            }
        });
    }
}
