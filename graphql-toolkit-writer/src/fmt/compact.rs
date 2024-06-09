use std::io;

use super::formatter::Formatter;
use crate::ser::{Serialize, Serializer};

/// A compact document formatter that writes GraphQL documents with no extra whitespace.
///
/// This is the default formatter used by this crate.
#[derive(Clone, Debug)]
pub struct CompactFormatter;

impl Formatter for CompactFormatter {}

impl<W> Serializer<W, CompactFormatter>
where
    W: io::Write,
{
    /// Create a new serializer that writes to the given I/O stream using the compact formatter.
    pub fn new(writer: W) -> Self {
        Serializer::with_formatter(writer, CompactFormatter)
    }
}

/// Serialize the given GraphQL AST as a compact GraphQL document into the I/O stream.
#[inline]
pub fn to_writer<W, T>(writer: W, value: &T) -> anyhow::Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}
/// Serialize the given GraphQL AST as a compact GraphQL document byte vector.
#[inline]
pub fn to_vec<T>(value: &T) -> anyhow::Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given GraphQL AST as a compact GraphQL document string.
#[inline]
pub fn to_string<T>(value: &T) -> anyhow::Result<String>
where
    T: ?Sized + Serialize,
{
    let vec = to_vec(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}
