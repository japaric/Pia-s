use js::Downcast;

use crate::Node;

js::inheritance!(Element: Node);

impl Element {
    pub fn set_class_name(&self, name: &js::String) {
        self.set(&"className".into(), name)
    }

    pub fn add_class(&self, name: &js::String) {
        js::call!(self.class_list(), add, name);
    }

    pub fn rm_class(&self, name: &js::String) {
        js::call!(self.class_list(), remove, name);
    }

    pub fn set_attribute(&self, name: &js::String, value: &js::Value) {
        js::call!(self, setAttribute, name, value);
    }

    pub fn set_id(&self, id: &js::String) {
        self.set(&"id".into(), id)
    }

    pub fn replace_children0(&self) {
        js::call!(self, replaceChildren);
    }

    fn class_list(&self) -> js::Object {
        unsafe { self.get(&"classList".into()).unwrap_unchecked().downcast() }
    }
}
