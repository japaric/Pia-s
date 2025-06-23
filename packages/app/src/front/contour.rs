use alloc::collections::btree_map::BTreeMap;
use alloc::format;
use alloc::vec::Vec;
use music::{Interval, Note};
use spur::{Message, Publish, React};
use web::{Node, Performance, SVGAnimateElement, SVGRectElement, SVGSVGElement};

use crate::broker::Broker;
use crate::class::Class;
use crate::messages::ActiveNotesChanged;
use crate::svg;

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
    current: Option<Note>,
    begin_zero: f64,
}

impl State {
    fn new(canvas: Canvas) -> Self {
        Self {
            canvas,
            current: None,
            begin_zero: Performance.now(),
        }
    }
}

impl React<ActiveNotesChanged> for Contour {
    fn react(
        &mut self,
        ActiveNotesChanged {
            held,
            sustained,
            timestamp,
        }: ActiveNotesChanged,
    ) {
        let Some(state) = &mut self.state else {
            return;
        };

        // `SVGAnimate.begin` does not share its "zero" reference with `performance.now`; instead
        // it (observally) treats its creation time as "zero". therefore we need to adjust the
        // timestamps obtained from `performance.now`
        let begin_timestamp = timestamp - state.begin_zero;

        let new_highest = held.highest();
        let last_highest = state.current;

        match (last_highest, new_highest) {
            (None, Some(new)) => {
                state.current = Some(new);

                state.canvas.on(new, begin_timestamp);
            }

            (Some(last), None) => {
                if !sustained.contains(last) {
                    state.current = None;

                    state.canvas.off(last, begin_timestamp);
                }
            }

            (Some(last), Some(new)) => {
                if last != new {
                    state.canvas.on(new, begin_timestamp);
                    state.canvas.off(last, begin_timestamp);

                    state.current = Some(new);
                }
            }

            (None, None) => {}
        }

        state.canvas.gc(begin_timestamp as i64);
    }
}

struct Canvas {
    root: SVGSVGElement,
    active: BTreeMap<Note, AnimatedLine>,
    notes: BTreeMap<i64, Vec<SVGRectElement>>,
}

struct AnimatedLine {
    translate: SVGAnimateElement,
    line: SVGRectElement,
    start: f64,
}

const CENTER: Note = Note::D5;
const DUR: f64 = 8.;

impl Canvas {
    fn new(parent: &SVGSVGElement) -> Self {
        parent.set_height(&js::String::from("300px"));
        parent.set_width(&js::String::from("100%"));

        let center = 150;

        let p8 = Interval::P8.as_half_steps() as i32 * 10;
        for offset in [-p8, 0, p8] {
            svg::rect(
                parent,
                Class::ContourGridMajor,
                &js::Integer::from(0),
                &js::Integer::from((center + offset) as u32),
                &js::String::from("100%"),
                &js::Integer::from(1),
            );
        }

        let p5 = Interval::P5.as_half_steps() as i32 * 10;
        for offset in [-p5, p5] {
            svg::rect(
                parent,
                Class::ContourGridMinor,
                &js::Integer::from(0),
                &js::Integer::from((center + offset) as u32),
                &js::String::from("100%"),
                &js::Integer::from(1),
            );
        }

        Self {
            root: parent.clone(),
            active: BTreeMap::new(),
            notes: BTreeMap::new(),
        }
    }

    fn on(&mut self, note: Note, now_ms: f64) {
        let half_steps = CENTER.distance_to(note) as i32;

        let line = svg::rect(
            &self.root,
            Class::ContourLine,
            &js::String::from("100%"),
            &js::Integer::from(150 - 10 * half_steps),
            &js::String::from("0"),
            &js::Integer::from(1),
        );
        line.set_rx(&js::Integer::from(3));
        line.set_ry(&js::Integer::from(3));

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

    fn gc(&mut self, now: i64) {
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
