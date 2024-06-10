//! A GraphQL document writer.

mod fmt;
mod ser;

pub use fmt::{
    compact::{to_string, to_vec, to_writer},
    formatter::Formatter,
    pretty::{to_string_pretty, to_vec_pretty, to_writer_pretty},
};
pub use ser::{Serialize, Serializer};
