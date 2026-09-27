//! @todo

/// @todo
pub fn process_data(data: Vec<u8>) -> Vec<u8> {
    // @todo
    let result = data.iter().map(|b| b.wrapping_add(1)).collect();
    result
}
