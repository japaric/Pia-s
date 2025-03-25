use music::{Note, Notes};
use spur::Message;

#[derive(Clone, Message)]
pub struct ActiveNotesChanged {
    pub held: Notes,
    pub sustained: Notes,
}

#[derive(Message)]
pub struct NewScaleTonicSelected(pub usize);

#[derive(Message)]
pub struct NewScaleTypeSelected(pub usize);

#[derive(Message)]
pub struct NoteOn(pub Note);

#[derive(Message)]
pub struct NoteOff(pub Note);

#[derive(Message)]
pub struct HoldPedalPressed;

#[derive(Message)]
pub struct HoldPedalReleased;
