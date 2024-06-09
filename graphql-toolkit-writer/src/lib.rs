//! A GraphQL document writer.

mod fmt;
mod ser;

pub use fmt::{
    compact::{to_string, to_vec, to_writer},
    formatter::Formatter,
};
pub use ser::{Serialize, Serializer};
