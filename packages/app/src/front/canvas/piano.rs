use alloc::vec::Vec;
use alloc::{string::ToString, vec};

use js::Integer;
use music::{Degree, Note, NoteName, Scale, ScaleType};
use scale_factor::ScaleFactor;
use web::{DominantBaseline, SVGRectElement, SVGSVGElement, SVGTextElement};

use crate::class::Class;
use crate::consts::MIN_KEY;
use crate::css::Percentage;
use crate::{consts, svg};

mod scale_factor;

const WHITE_KEY_HEIGHT: &str = "100%";
const WHITE_KEY_TEXT_HEIGHT: &str = "80%";
const BLACK_KEY_HEIGHT: &str = "60%";
const BLACK_KEY_TEXT_HEIGHT: &str = "30%";

pub struct Piano {
    labeled_keys: Vec<Key>,
}

#[derive(Clone)]
struct Key {
    back: SVGRectElement,
    front: SVGRectElement,
    label: SVGTextElement,
}

impl Piano {
    pub fn new(parent: &SVGSVGElement) -> Self {
        let num_keys = consts::MAX_KEY.as_u8() + 1 - consts::MIN_KEY.as_u8();
        let mut keys = vec![None; num_keys as usize];

        let num_white_keys = notes().filter(|note| note.is_natural()).count();

        let scale_factor = ScaleFactor::new(num_white_keys);
        let white_key_width = scale_factor.white_key_width();
        let white_key_width_js = js::String::from(Percentage(white_key_width).to_string().as_str());

        let white_key_height_js = js::String::from(WHITE_KEY_HEIGHT);
        let white_key_text_height_js = js::String::from(WHITE_KEY_TEXT_HEIGHT);

        for (index, note) in notes().filter(|note| note.is_natural()).enumerate() {
            let x = index as f64 * white_key_width;
            let rect_x = js::String::from(Percentage(x).to_string().as_str());
            let back_rect = svg::rect(
                parent,
                Class::PianoWhite,
                &rect_x,
                &Integer::from(0),
                &white_key_width_js,
                &white_key_height_js,
            );
            let front_rect = svg::rect(
                parent,
                Class::PianoColor,
                &rect_x,
                &Integer::from(0),
                &white_key_width_js,
                &white_key_height_js,
            );

            let text_x_js =
                js::String::from(Percentage(x + white_key_width / 2.).to_string().as_str());
            let text = svg::text(parent, &text_x_js, &white_key_text_height_js);
            text.set_class_name(&Class::Degree.as_str().into());

            if note.name() == NoteName::C {
                let text = svg::text(parent, &text_x_js, &white_key_height_js);
                text.set_class_name(&Class::Octave.as_str().into());
                text.set_dominant_baseline(DominantBaseline::TextTop);
                text.set_text_content(&circled_octave(note.octave()));
            }

            keys[note2index(note)] = Some(Key {
                back: back_rect,
                front: front_rect,
                label: text,
            });
        }

        let black_key_height_js = js::String::from(BLACK_KEY_HEIGHT);
        let black_key_text_height_js = js::String::from(BLACK_KEY_TEXT_HEIGHT);

        let black_key_width = scale_factor.black_key_width();
        let black_key_width_js = js::String::from(Percentage(black_key_width).to_string().as_str());
        let mut rect_x = scale_factor.offset(consts::MIN_KEY.name());
        for note in notes() {
            let width = scale_factor.width(note.name());
            if note.is_natural() {
                rect_x += width;
                continue;
            }

            let rect_x_js = js::String::from(Percentage(rect_x).to_string().as_str());
            // put a black rectangle behind so we can apply opacity here in the same way we can
            // apply it to the white keys
            let back_rect = svg::rect(
                parent,
                Class::PianoBlack,
                &rect_x_js,
                &Integer::from(0),
                &black_key_width_js,
                &black_key_height_js,
            );
            let front_rect = svg::rect(
                parent,
                Class::PianoColor,
                &rect_x_js,
                &Integer::from(0),
                &black_key_width_js,
                &black_key_height_js,
            );

            let text_x = rect_x + black_key_width / 2.;
            let text_x_js = js::String::from(Percentage(text_x).to_string().as_str());
            let text = svg::text(parent, &text_x_js, &black_key_text_height_js);
            text.set_class_name(&Class::Degree.as_str().into());

            rect_x += width;
            keys[note2index(note)] = Some(Key {
                back: back_rect,
                front: front_rect,
                label: text,
            });
        }

        let piano = Self {
            labeled_keys: keys.into_iter().flatten().collect(),
        };
        piano.set_scale(
            NoteName::CIRCLE_OF_FIFTHS[consts::INITIAL_SCALE_TONIC_INDEX as usize],
            ScaleType::ALL[consts::INITIAL_SCALE_TYPE_INDEX as usize],
        );
        piano
    }

    pub fn set_scale(&self, tonic: NoteName, scale_ty: ScaleType) {
        let scale = Scale::new(tonic);

        for (note, key) in notes().zip(&self.labeled_keys) {
            let degree = scale.name2degree(note.name());
            key.label.set_text_content(&degree.as_str().into());

            for other in Degree::ALL {
                if degree == other {
                    key.front.add_class(&degree.as_str().into());
                } else {
                    key.front.rm_class(&other.as_str().into());
                }
            }

            let class_js = js::String::from(Class::OutOfKey.as_str());
            if degree.belongs_to(scale_ty) {
                key.back.rm_class(&class_js);
            } else {
                key.back.add_class(&class_js);
            }
        }
    }

    pub fn pressed(&self, note: Note) {
        if let Some(key) = self.get(note) {
            let class = js::String::from(Class::Pressed.as_str());
            key.label.add_class(&class);
            key.front.add_class(&class);
        }
    }

    pub fn released(&self, note: Note) {
        if let Some(key) = self.get(note) {
            let class = js::String::from(Class::Pressed.as_str());
            key.label.rm_class(&class);
            key.front.rm_class(&class);
        }
    }

    pub fn sustain_on(&self, note: Note) {
        if let Some(key) = self.get(note) {
            let class = js::String::from(Class::Sustained.as_str());
            key.label.add_class(&class);
            key.front.add_class(&class);
            key.back.add_class(&class);
        }
    }

    pub fn sustain_off(&self, note: Note) {
        if let Some(key) = self.get(note) {
            let class = js::String::from(Class::Sustained.as_str());
            key.label.rm_class(&class);
            key.front.rm_class(&class);
            key.back.rm_class(&class);
        }
    }

    pub fn overtone_on(&self, note: Note, power: f64) {
        if let Some(key) = self.get(note) {
            let class = js::String::from(Class::Overtone.as_str());
            key.front.add_class(&class);
            let opacity = 0.8 * power;
            key.front.set_opacity(&js::Float::from(opacity));
        }
    }

    pub fn overtone_off(&self, note: Note) {
        if let Some(key) = self.get(note) {
            let class = js::String::from(Class::Overtone.as_str());
            key.front.rm_class(&class);
        }
    }

    fn get(&self, note: Note) -> Option<&Key> {
        self.labeled_keys.get(note2index(note))
    }
}

fn note2index(note: Note) -> usize {
    (note.as_u8() - MIN_KEY.as_u8()) as usize
}

fn circled_octave(octave: i8) -> js::String {
    match octave {
        0 => "⓪",
        1 => "①",
        2 => "②",
        3 => "③",
        4 => "④",
        5 => "⑤",
        6 => "⑥",
        7 => "⑦",
        8 => "⑧",
        9 => "⑨",
        _ => unimplemented!(),
    }
    .into()
}

fn notes() -> impl Iterator<Item = Note> {
    (consts::MIN_KEY.as_u8()..=consts::MAX_KEY.as_u8()).map(Note::from_u8_lossy)
}
