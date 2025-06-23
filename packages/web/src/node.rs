use crate::EventTarget;

js::inheritance!(Node: EventTarget);

impl Node {
    pub fn append_child(&self, child: &Node) {
        js::call!(self, appendChild, child);
    }

    pub fn remove_child(&self, child: &Node) {
        js::call!(self, removeChild, child);
    }

    pub fn set_text_content(&self, text: &js::String) {
        self.set(&"textContent".into(), text);
    }
}
