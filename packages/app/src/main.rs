#![no_std]
#![no_main]

extern crate alloc;

use web::{Body, MIDIAccess, Navigator};

use crate::class::Class;

mod back;
mod broker;
mod class;
mod consts;
mod css;
mod front;
mod heap;
mod html;
mod messages;
#[cfg(not(test))]
mod panic;
mod svg;

#[unsafe(no_mangle)]
extern "C" fn _start() {
    heap::initialize();

    html::div(&Body, Class::HvCenter).set_text_content(&"waiting for MIDI permissions..".into());

    Navigator.request_midi_access().then2(fulfilled, rejected);
}

fn fulfilled(midi_access: MIDIAccess) {
    Body.replace_children0();

    let div = html::div(&Body, Class::RowContainer);
    front::initialize(&div, midi_access);
}

fn rejected() {
    Body.replace_children0();

    html::div(&Body, Class::HvCenter)
        .set_text_content(&"access to MIDI devices was denied :-(".into());
}
