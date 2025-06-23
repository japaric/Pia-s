use music::Notes;
use spur::{Publish as _, React};

use crate::broker::Broker;
use crate::messages::{ActiveNotesChanged, HoldPedalPressed, HoldPedalReleased, NoteOff, NoteOn};

pub struct NoteGrouper {
    held: Notes,
    sustain: bool,
    sustained: Notes,
}

impl NoteGrouper {
    pub const fn new() -> Self {
        Self {
            held: Notes::empty(),
            sustain: false,
            sustained: Notes::empty(),
        }
    }

    fn publish(&self) {
        Broker::publish(ActiveNotesChanged {
            held: self.held.clone(),
            sustained: self.sustained.difference(&self.held),
        })
    }
}

impl React<NoteOff> for NoteGrouper {
    fn react(&mut self, NoteOff(note, _): NoteOff) {
        self.held.remove(note);
        self.publish();
    }
}

impl React<NoteOn> for NoteGrouper {
    fn react(&mut self, NoteOn(note, _): NoteOn) {
        self.held.insert(note);
        if self.sustain {
            self.sustained.insert(note);
        }
        self.publish();
    }
}

impl React<HoldPedalPressed> for NoteGrouper {
    fn react(&mut self, _message: HoldPedalPressed) {
        self.sustain = true;
        self.sustained = self.held.clone();
    }
}

impl React<HoldPedalReleased> for NoteGrouper {
    fn react(&mut self, _: HoldPedalReleased) {
        self.sustain = false;
        self.sustained.clear();
        self.publish();
    }
}
