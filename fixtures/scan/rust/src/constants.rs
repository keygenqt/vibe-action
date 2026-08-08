/// Global constants
pub const MAX_SIZE: usize = 1024;

/// TEMP: remove after migration
const DEFAULT_TIMEOUT: u64 = 30;

/// Static variables
pub static APP_NAME: &str = "vibe-action";

/// WORKAROUND: mut static for legacy
static mut COUNTER: u32 = 0;
