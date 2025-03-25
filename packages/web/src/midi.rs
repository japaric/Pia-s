use js::{Downcast as _, Function, Uint8Array};

use crate::{Event, EventTarget};

js::inheritance!(MIDIAccess: EventTarget);

impl MIDIAccess {
    pub fn inputs(&self) -> MIDIInputMap {
        unsafe { self.get(&"inputs".into()).unwrap_unchecked().downcast() }
    }
}

js::inheritance!(MIDIInputMap: js::Map);

impl MIDIInputMap {
    pub fn entries(&self) -> impl Iterator<Item = (js::String, MIDIInput)> {
        self.inner.entries().filter_map(|value| {
            let array: js::Array = value.downcast();

            Some((array.at(0)?.downcast(), array.at(1)?.downcast()))
        })
    }

    pub fn get(&self, key: &js::String) -> Option<MIDIInput> {
        self.inner.get(key).map(|value| value.downcast())
    }
}

js::inheritance!(MIDIInput: MIDIPort);

impl MIDIInput {
    pub fn set_onmidimessage(&self, callback: Option<fn(MIDIMessageEvent)>) {
        let property = "onmidimessage".into();
        if let Some(f) = callback {
            self.set(&property, &Function::from(f));
        } else {
            self.set(&property, &js::Null);
        }
    }
}

js::inheritance!(MIDIPort: EventTarget);

impl MIDIPort {
    pub fn name(&self) -> js::String {
        unsafe { self.get(&"name".into()).unwrap_unchecked().downcast() }
    }
}

js::inheritance!(MIDIMessageEvent: Event);

impl MIDIMessageEvent {
    pub fn data(&self) -> Uint8Array {
        unsafe { self.get(&"data".into()).unwrap_unchecked().downcast() }
    }
}
