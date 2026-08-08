/// User structure with fields
#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    age: u32,
    email: String,
}

/// Internal struct
struct Config {
    // BUG: debug flag not used
    debug: bool,
    max_size: usize,
}
