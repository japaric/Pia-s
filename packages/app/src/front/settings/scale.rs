use alloc::vec::Vec;
use js::{Downcast as _, Upcast as _};
use music::{NoteName, Scale, ScaleType};
use spur::{Message, Publish as _, React};
use web::{HtmlOptionElement, HtmlSelectElement};

use crate::broker::Broker;
use crate::html::Form;
use crate::messages::{NewScaleTonicSelected, NewScaleTypeSelected};
use crate::{consts, html};

pub(super) fn initialize(form: &Form) {
    let fieldset = form.fieldset(&"Scale".into());

    let tonic_select = html::select(&fieldset, &"scale-tonic".into());
    tonic_select.set_required(true);

    let mut options = Vec::with_capacity(12);
    for note in NoteName::CIRCLE_OF_FIFTHS {
        let option = html::option(&tonic_select, None, &note.as_flat_str().into());
        options.push(option);
    }

    tonic_select.set_selected_index(consts::INITIAL_SCALE_TONIC_INDEX.into());
    tonic_select.set_onchange(|event| onchange_tonic(event.target().upcast().upcast().downcast()));

    let type_select = html::select(&fieldset, &"scale-type".into());
    type_select.set_required(true);

    for ty in ScaleType::ALL {
        html::option(&type_select, None, &ty.as_str().into());
    }

    type_select.set_selected_index(consts::INITIAL_SCALE_TYPE_INDEX.into());
    type_select.set_onchange(|event| onchange_type(event.target().upcast().upcast().downcast()));

    Broker::publish(Initialize(State { options }));
}

pub struct ScaleTonicSelect {
    state: Option<State>,
}

impl ScaleTonicSelect {
    pub const fn new() -> Self {
        Self { state: None }
    }
}

#[derive(Message)]
pub struct Initialize(State);

impl React<Initialize> for ScaleTonicSelect {
    fn react(&mut self, Initialize(state): Initialize) {
        self.state = Some(state);
    }
}

impl React<NewScaleTypeSelected> for ScaleTonicSelect {
    fn react(&mut self, NewScaleTypeSelected(index): NewScaleTypeSelected) {
        let Some(State { options }) = &mut self.state else {
            return;
        };

        let ty = ScaleType::ALL[index];

        for (option, note) in options.iter_mut().zip(NoteName::CIRCLE_OF_FIFTHS) {
            option.set_text_content(&js::String::from(note.as_str(Scale { tonic: note, ty })));
        }
    }
}

struct State {
    options: Vec<HtmlOptionElement>,
}

fn onchange_tonic(select: HtmlSelectElement) {
    let selected = select.selected_index();
    let Ok(index) = selected.try_into() else {
        return;
    };

    Broker::publish(NewScaleTonicSelected(index));
}

fn onchange_type(select: HtmlSelectElement) {
    let selected = select.selected_index();
    let Ok(index) = selected.try_into() else {
        return;
    };

    Broker::publish(NewScaleTypeSelected(index));
}
