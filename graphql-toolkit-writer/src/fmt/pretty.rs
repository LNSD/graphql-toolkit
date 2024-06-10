use std::io;

use super::formatter::Formatter;
use crate::ser::{Serialize, Serializer};

/// This formatter generates a GraphQL document with a human-readable format.
#[derive(Clone, Debug)]
pub struct PrettyFormatter<'a> {
    current_indent_level: usize,
    indent: &'a [u8],
}

impl<'a> PrettyFormatter<'a> {
    /// Construct a pretty printer formatter that defaults to using two spaces for indentation.
    pub fn new() -> Self {
        Default::default()
    }

    /// Construct a pretty printer formatter that uses the `indent` string for indentation.
    pub fn with_indent(indent: &'a [u8]) -> Self {
        PrettyFormatter {
            current_indent_level: 0,
            indent,
        }
    }
}

impl<'a> Default for PrettyFormatter<'a> {
    fn default() -> Self {
        PrettyFormatter::with_indent(b"  ")
    }
}

impl<'a> Formatter for PrettyFormatter<'a> {
    #[inline]
    fn before_operation_or_fragment_definition<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\n")
    }

    #[inline]
    fn before_operation_variable_definitions<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b" ")
    }

    #[inline]
    fn after_operation_or_fragment_signature<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b" ")
    }

    #[inline]
    fn after_selection_signature<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b" ")
    }

    #[inline]
    fn before_type_condition<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b" ")
    }

    #[inline]
    fn before_directive<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b" ")
    }

    fn write_name_value_separator<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b": ")
    }

    fn write_variable_default_value_separator<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b" = ")
    }

    fn write_item_separator<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b", ")
    }

    #[inline]
    fn begin_block<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent_level += 1;

        writer.write_all(b"{\n")
    }

    #[inline]
    fn end_block<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\n")?;

        self.current_indent_level -= 1;
        indent(writer, self.current_indent_level, self.indent)?;

        writer.write_all(b"}")
    }

    #[inline]
    fn before_block_item<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        indent(writer, self.current_indent_level, self.indent)
    }

    #[inline]
    fn after_block_item<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\n")
    }
}

/// Indent the specified writer `n` times by the `s` number of spaces.
fn indent<W>(writer: &mut W, n: usize, indent: &[u8]) -> io::Result<()>
where
    W: ?Sized + io::Write,
{
    for _ in 0..n {
        writer.write_all(indent)?;
    }

    Ok(())
}

/// Serialize the given GraphQL AST as a pretty-printed GraphQL document into the I/O stream.
#[inline]
pub fn to_writer_pretty<W, T>(writer: W, value: &T) -> anyhow::Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::with_formatter(writer, PrettyFormatter::new());
    value.serialize(&mut ser)
}

/// Serialize the given GraphQL AST as a pretty-printed GraphQL document byte vector.
#[inline]
pub fn to_vec_pretty<T>(value: &T) -> anyhow::Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer_pretty(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given GraphQL AST as a pretty-printed GraphQL document string.
#[inline]
pub fn to_string_pretty<T>(value: &T) -> anyhow::Result<String>
where
    T: ?Sized + Serialize,
{
    let vec = to_vec_pretty(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}
