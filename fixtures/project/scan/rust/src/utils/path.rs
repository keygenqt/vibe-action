use std::path::Path;

// TODO: Implement path validation
pub fn validate(p: &Path) -> bool {
    p.exists()
}
