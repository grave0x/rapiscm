//! Tracker detection, analytics classification, cookie analysis, and device profiling.

mod cookies;
mod detect;
mod export;
mod profile;

// Re-export detect (the core tracker detection) always.
pub use detect::*;
// Cookie, export, and profile functions are available via their modules.
pub use cookies::{CookiePurpose, analyze_cookie_security, classify_cookie};
