use core::fmt;

// TODO do not rely on float formatting at all and use floats, instead of strings, for width/height
#[derive(Clone, Copy)]
pub struct Percentage(pub f64);

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0 * 100.)
    }
}

#[derive(Clone, Copy)]
pub struct Em(pub u32);

impl fmt::Display for Em {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}em", self.0)
    }
}
