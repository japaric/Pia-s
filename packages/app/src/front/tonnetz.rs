use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;

use js::Float;
use music::{Degree, Degrees, MajorScale, NoteName, NoteNames, ScaleType};
use spur::{Message, Publish as _, React};
use web::{Node, SVGEllipseElement, SVGSVGElement, SVGTextElement};

use crate::broker::Broker;
use crate::class::Class;
use crate::messages::{ActiveNotesChanged, NewScaleTonicSelected, NewScaleTypeSelected};
use crate::{consts, svg};

pub fn initialize(parent: &Node) {
    let canvas = Canvas::new(&svg::svg(parent, Class::Tonnetz, false));

    Broker::publish(Initialize { canvas });
}

pub struct Tonnetz {
    state: Option<State>,
}

impl Tonnetz {
    pub const fn new() -> Self {
        Self { state: None }
    }
}

#[derive(Message)]
pub struct Initialize {
    canvas: Canvas,
}

impl React<Initialize> for Tonnetz {
    fn react(&mut self, Initialize { canvas }: Initialize) {
        let tonic = NoteName::CIRCLE_OF_FIFTHS[consts::INITIAL_SCALE_TONIC_INDEX as usize];
        let scale_type = ScaleType::ALL[consts::INITIAL_SCALE_TYPE_INDEX as usize];
        canvas.set_scale_type(scale_type);
        self.state = Some(State {
            canvas,
            live: NoteNames::empty(),
            scale: MajorScale::new(tonic),
        })
    }
}

impl React<NewScaleTonicSelected> for Tonnetz {
    fn react(&mut self, NewScaleTonicSelected(index): NewScaleTonicSelected) {
        let Some(state) = &mut self.state else { return };

        let new_tonic = NoteName::CIRCLE_OF_FIFTHS[index];

        if new_tonic != state.scale.tonic() {
            state.scale = MajorScale::new(new_tonic);

            let mut degrees = Degrees::empty();
            for note in state.live.clone() {
                let degree = state.scale.name2degree(note);
                degrees.insert(degree);
            }

            state.canvas.highlight(degrees);
        }
    }
}

impl React<NewScaleTypeSelected> for Tonnetz {
    fn react(&mut self, NewScaleTypeSelected(index): NewScaleTypeSelected) {
        let Some(state) = &mut self.state else { return };

        let new_scale_type = ScaleType::ALL[index];

        state.canvas.reset_in_key();
        state.canvas.set_scale_type(new_scale_type);
    }
}

impl React<ActiveNotesChanged> for Tonnetz {
    fn react(
        &mut self,
        ActiveNotesChanged {
            held, sustained, ..
        }: ActiveNotesChanged,
    ) {
        let Some(state) = &mut self.state else { return };

        let mut notes = NoteNames::empty();
        for note in held.into_iter().chain(sustained) {
            notes.insert(note.name());
        }

        let mut degrees = Degrees::empty();
        for note in notes.clone() {
            let degree = state.scale.name2degree(note);
            degrees.insert(degree);
        }

        state.canvas.highlight(degrees);

        state.live = notes;
    }
}

struct State {
    canvas: Canvas,
    live: NoteNames,
    scale: MajorScale,
}

struct Canvas {
    items: BTreeMap<Degree, Vec<Item>>,
}

#[derive(Clone)]
struct Item {
    // coordinates relative to canvas center
    cx: f64,
    cy: f64,
    circle: SVGEllipseElement,
    label: SVGTextElement,
}

impl Canvas {
    fn new(parent: &SVGSVGElement) -> Self {
        const REACH: usize = 3;

        use Degree::*;

        const PADDING: f64 = 0.5;
        const HEIGHT_PX: f64 = 300.;
        const GR: f64 = 1.618033988749895;
        const SQRT3: f64 = 1.7320508075688772;

        let dy = (GR + 1.) / 2. * SQRT3;
        let dx = GR + 1.;
        let height = 2. * REACH as f64 * dy + 2. + 2. * PADDING;
        let width = 2. * REACH as f64 * dx + 2. + 2. * PADDING;

        let r_px = HEIGHT_PX / height;
        let width_px = r_px * width;
        parent.set_width(&Float::from(width_px));

        let mut items = BTreeMap::<_, Vec<_>>::new();

        let label_class = Class::TonnetzLabel.as_str().into();

        let center_x = width_px / 2.;
        let center_y = HEIGHT_PX / 2.;

        for curr_row in -3isize..=3 {
            let num_cols = 7 - curr_row.abs();
            let mut degree = if curr_row % 2 == 0 {
                One
            } else {
                One.step(if curr_row < 0 { 3 } else { 4 })
            }
            .step(curr_row / 2)
            .step(-7 * (num_cols / 2));
            let start_col = 0.5 - num_cols as f64 / 2.;

            let mut curr_col = start_col;
            for _ in 0..num_cols {
                let offset_x = curr_col * dx;
                let offset_y = curr_row as f64 * dy;

                let cx_px = (center_x + offset_x * r_px).into();
                let cy_px = (center_y + offset_y * r_px).into();
                let circle = svg::circle(parent, Class::TonnetzCircle, &cx_px, &cy_px, r_px);

                let label = svg::text(parent, &cx_px, &cy_px);
                label.set_class_name(&label_class);
                label.add_class(&js::String::from(degree.as_str()));
                label.set_text_content(&degree.as_str().into());

                let item = Item {
                    cx: offset_x,
                    cy: offset_y,
                    circle,
                    label,
                };
                items.entry(degree).or_default().push(item);

                degree = degree.step(7);
                curr_col += 1.;
            }
        }

        Self { items }
    }

    fn reset_highlights(&self) {
        let class = Class::Highlight.as_str().into();
        for items in self.items.values() {
            for item in items {
                item.circle.rm_class(&class);
                item.label.rm_class(&class);
            }
        }
    }

    fn highlight(&self, mut degrees: Degrees) {
        self.reset_highlights();

        let class = Class::Highlight.as_str().into();

        let mut centers = Vec::with_capacity(degrees.len());
        while !degrees.is_empty() {
            let (degree, item) = self.closest(degrees.clone(), &centers);

            item.circle.add_class(&class);
            item.label.add_class(&class);
            centers.push((item.cx, item.cy));

            degrees.remove(degree);
        }
    }

    fn closest(&self, degrees: Degrees, centers: &[(f64, f64)]) -> (Degree, Item) {
        let mut closest: Option<(f64, f64, _, _)> = None;
        for degree in degrees.clone() {
            for item in self.items[&degree].iter() {
                let distances = if centers.is_empty() {
                    item.cx * item.cx + item.cy * item.cy
                } else {
                    centers
                        .iter()
                        .map(|(cx, cy)| {
                            let dx = cx - item.cx;
                            let dy = cy - item.cy;

                            dx * dx + dy * dy
                        })
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                };

                let distance_to_item = distances;
                let distance_to_center = item.cx * item.cx + item.cy * item.cy;
                if let Some(current) = &mut closest {
                    if distance_to_item < current.0
                        || distance_to_item == current.0 && distance_to_center < current.1
                    {
                        *current = (distance_to_item, distance_to_center, degree, item.clone());
                    }
                } else {
                    closest = Some((distance_to_item, distance_to_center, degree, item.clone()));
                }
            }
        }

        let (_, _, degree, item) = closest.unwrap();

        (degree, item)
    }

    fn set_scale_type(&self, scale_type: ScaleType) {
        let class = Class::InScale.as_str().into();
        for degree in scale_type.degrees() {
            for item in &self.items[degree] {
                item.circle.add_class(&class);
            }
        }
    }

    fn reset_in_key(&self) {
        let class = Class::InScale.as_str().into();
        for items in self.items.values() {
            for item in items {
                item.circle.rm_class(&class);
            }
        }
    }
}
