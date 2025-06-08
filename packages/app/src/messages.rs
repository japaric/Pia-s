use alloc::collections::btree_map::BTreeMap;
use music::{Note, NoteName, Notes};
use spur::Message;

#[derive(Clone, Message)]
pub struct ActiveNotesChanged {
    pub held: Notes,
    pub sustained: Notes,
}

#[derive(Clone, Message)]
pub struct ActiveHarmonyChanged {
    pub tonics: BTreeMap<NoteName, /* is_minor */ bool>,
}

#[derive(Clone, Message)]
pub struct NewScaleTonicSelected(pub usize);

#[derive(Clone, Message)]
pub struct NewScaleTypeSelected(pub usize);

#[derive(Message)]
pub struct NoteOn(pub Note);

#[derive(Message)]
pub struct NoteOff(pub Note);

#[derive(Message)]
pub struct HoldPedalPressed;

#[derive(Message)]
pub struct HoldPedalReleased;
