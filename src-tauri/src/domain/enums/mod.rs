use serde::{ Serialize, Deserialize };
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub mod method_enum;
pub mod header_enum;
pub mod parameter_enum;

pub use method_enum::HttpMethod;
pub use header_enum::HttpHeader;
pub use parameter_enum::HttpParameter;
