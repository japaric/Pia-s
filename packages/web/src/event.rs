use js::Downcast;

js::inheritance!(EventTarget: js::Object);

js::inheritance!(Event: js::Object);

impl Event {
    pub fn target(&self) -> EventTarget {
        unsafe { self.get(&"target".into()).unwrap_unchecked().downcast() }
    }

    pub fn timestamp(&self) -> f64 {
        unsafe {
            let x: js::Float = self.get(&"timeStamp".into()).unwrap_unchecked().downcast();
            x.into()
        }
    }
}
