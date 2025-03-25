use js::{Downcast as _, Upcast as _};
use music::{NoteName, ScaleType};
use spur::Publish as _;
use web::HtmlSelectElement;

use crate::broker::Broker;
use crate::html::Form;
use crate::messages::{NewScaleTonicSelected, NewScaleTypeSelected};
use crate::{consts, html};

pub(super) fn initialize(form: &Form) {
    let fieldset = form.fieldset(&"Scale".into());

    let tonic_select = html::select(&fieldset, &"scale-tonic".into());
    tonic_select.set_required(true);

    for note in NoteName::CIRCLE_OF_FIFTHS {
        html::option(&tonic_select, None, &note.as_flat_str().into());
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
