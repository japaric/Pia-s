use js::Downcast;

js::inheritance!(EventTarget: js::Object);

js::inheritance!(Event: js::Object);

impl Event {
    pub fn target(&self) -> EventTarget {
        unsafe { self.get(&"target".into()).unwrap_unchecked().downcast() }
    }
}
