/// Color enum
pub enum Color {
    // NOTE: add more colors later
    Red,
    Green,
    Blue,
}

/// Display trait
pub trait Display {
    fn display(&self) -> String;
}

/// Debug trait
pub trait Debug {
    // OPTIMIZE: too slow
    fn debug(&self);
}

/// REVIEW: do we need this?
pub type MyResult<T> = Result<T, String>;
