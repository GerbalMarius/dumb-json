mod djson;
pub mod lexer;
pub mod traits;

pub use traits::encode::Encode;
pub use traits::decode::Decode;

pub use djson::*;

pub use djson_derive::Encode;
pub use djson_derive::Decode;

//helper literals
#[cfg(target_os = "windows")]
const NEW_LINE: &str = "\r\n";

#[cfg(target_os = "linux")]
const NEW_LINE: &str = "\n";

#[cfg(target_os = "macos")]
const NEW_LINE: &str = "\r";
