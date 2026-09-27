/// Adds two numbers
fn add(a: i32, b: i32) -> i32 {
    // TODO: internal comment - ignored
    a + b
}

/// Multiplies two numbers
pub fn multiply(a: i32, b: i32) -> i32 {
    // HACK: temporary fix
    a * b
}

/// Async function example
pub async fn fetch_data(url: &str) -> Result<String, std::io::Error> {
    // XXX: what if url is empty?
    Ok(String::new())
}

/// Unsafe function example
pub unsafe fn dangerous() {
    // @todo unsafe code
}
