pub mod charts;
pub mod confirmation;
pub mod home;
mod input;
pub mod login;
pub mod practices;
pub mod pwd_reset;
pub mod register_with_id;
pub mod settings;
pub mod user_practices;
pub mod yatras;

pub use input::*;

const DROPDOWN_PRACTICE_TYPES: &[&str] = &["int", "text"];
