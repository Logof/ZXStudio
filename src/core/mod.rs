pub mod io;
pub mod validator;

pub use io::{save_project_json, export_config_h};
pub use validator::{validate_attribute_clash, ClashError};
