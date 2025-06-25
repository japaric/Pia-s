use alloc::collections::btree_map::BTreeMap;
use alloc::format;
use alloc::vec::Vec;
use music::{MajorScale, Note, NoteName, Notes};
use spur::{Message, Publish, React};
use web::{Node, Performance, SVGAnimateElement, SVGRectElement, SVGSVGElement};

use crate::broker::Broker;
use crate::class::Class;
use crate::messages::{NewScaleTonicSelected, NoteOff, NoteOn};
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

impl React<NewScaleTonicSelected> for Contour {
    fn react(&mut self, NewScaleTonicSelected(index): NewScaleTonicSelected) {
        let Some(state) = &mut self.state else {
            return;
        };

        let new_tonic = NoteName::CIRCLE_OF_FIFTHS[index];
        if state.canvas.scale.tonic() != new_tonic {
            state.canvas.scale = MajorScale::new(new_tonic);
            state.canvas.redraw_grid();
            state.canvas.gc(Performance.now() - state.begin_zero);
            state.canvas.recolor_lines();
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

        state.canvas.maybe_pan(state.begin_zero);
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
    active: BTreeMap<Note, AnimatedLine>,
    current_y: u32,
    grid: Vec<SVGRectElement>,
    lines: BTreeMap<i64, Vec<(Note, SVGRectElement)>>,
    next_pan: f64,
    root: SVGSVGElement,
    scale: MajorScale,
    view_box_animate: Option<SVGAnimateElement>,
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
const HEIGHT: u32 = 300;
const PAN_COOLDOWN: f64 = 1000.; // ms
const WIDTH: u32 = 800;
const PAN_DUR: &str = "1s";

impl Canvas {
    fn new(parent: &SVGSVGElement) -> Self {
        let scale =
            MajorScale::new(NoteName::CIRCLE_OF_FIFTHS[consts::INITIAL_SCALE_TONIC_INDEX as usize]);

        parent.set_height(&js::String::from("300px"));
        parent.set_width(&js::String::from("100%"));
        let y = note2y(Note::C4);
        parent.set_view_box(&js::String::from(
            format!("0 {y} {WIDTH} {HEIGHT}").as_str(),
        ));

        let mut this = Self {
            active: BTreeMap::new(),
            current_y: y,
            grid: Vec::new(),
            lines: BTreeMap::new(),
            next_pan: 0.,
            root: parent.clone(),
            scale,
            view_box_animate: None,
        };

        this.redraw_grid();

        this
    }

    fn maybe_pan(&mut self, begin_zero: f64) {
        let svg_now = Performance.now() - begin_zero;

        if self.next_pan > svg_now {
            return;
        }

        let mut active = Notes::empty();
        for pairs in self.lines.values() {
            for (note, _) in pairs {
                active.insert(*note);
            }
        }

        if active.len() < 3 {
            return;
        }

        let highest = active.highest().unwrap();

        let tonic = self.scale.tonic();
        let highest_octave = highest.octave();
        let mut tonic_below = tonic.with_octave(highest_octave).unwrap();
        if tonic_below >= highest {
            tonic_below = tonic.with_octave(highest_octave - 1).unwrap();
        }
        let next_y = note2y(tonic_below);

        if next_y == self.current_y {
            return;
        }

        if let Some(old_animate) = self.view_box_animate.take() {
            self.root.remove_child(&old_animate);
        }

        let begin = &js::String::from(format!("{}s", svg_now / 1000.).as_str());
        let from = &js::String::from(format!("0 {} {WIDTH} {HEIGHT}", self.current_y).as_str());
        let dur = &js::String::from(PAN_DUR);
        let to = &js::String::from(format!("0 {} {WIDTH} {HEIGHT}", next_y).as_str());

        let animate = svg::animate(
            &self.root,
            &js::String::from("viewBox"),
            begin,
            dur,
            from,
            to,
        );
        animate.set_fill(&js::String::from("freeze"));

        self.view_box_animate = Some(animate);

        self.current_y = next_y;

        self.next_pan = svg_now + PAN_COOLDOWN;
    }

    fn redraw_grid(&mut self) {
        let tonic = self.scale.tonic();

        for line in &self.grid {
            self.root.remove_child(line);
        }
        self.grid.clear();

        for octave in 0..9 {
            let Ok(p0) = tonic.with_octave(octave) else {
                continue;
            };

            if p0 >= MIN_NOTE || p0 <= MAX_NOTE {
                let y = p0.distance_to(MAX_NOTE) as u32 * SEMITONE_GAP + SEMITONE_GAP / 2;

                let line = svg::rect(
                    &self.root,
                    Class::ContourGridMajor,
                    &js::Integer::from(0),
                    &js::Integer::from(y),
                    &js::String::from("100%"),
                    &js::Integer::from(1),
                );
                self.grid.push(line);
            }

            let Ok(p5) = p0.step(7) else { continue };

            if p0 >= MIN_NOTE || p0 <= MAX_NOTE {
                let y = p5.distance_to(MAX_NOTE) as u32 * SEMITONE_GAP + SEMITONE_GAP / 2;

                let line = svg::rect(
                    &self.root,
                    Class::ContourGridMinor,
                    &js::Integer::from(0),
                    &js::Integer::from(y),
                    &js::String::from("100%"),
                    &js::Integer::from(1),
                );
                self.grid.push(line);
            }
        }
    }

    fn recolor_lines(&self) {
        for pairs in self.lines.values() {
            for (note, line) in pairs {
                let degree = self.scale.name2degree(note.name());
                line.add_class(&js::String::from(degree.as_str()));
            }
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
            self.lines.entry(deadline).or_default().push((note, line));
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
            self.lines.entry(deadline).or_default().push((note, line));
        };
    }

    fn gc(&mut self, now: f64) {
        let now = now as i64;
        self.lines.retain(|deadline, lines| {
            if *deadline > now {
                true
            } else {
                for (_note, line) in lines {
                    self.root.remove_child(line);
                }
                false
            }
        });
    }
}

fn note2y(note: Note) -> u32 {
    note.distance_to(MAX_NOTE) as u32 * SEMITONE_GAP + SEMITONE_GAP / 2 - HEIGHT / 2
}
