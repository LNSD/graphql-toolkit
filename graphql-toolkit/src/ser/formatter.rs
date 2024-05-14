use std::io;

pub trait Formatter {
    #[inline]
    fn write_keyword<W>(&mut self, writer: &mut W, name: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(name.as_bytes())
    }

    #[inline]
    fn begin_operation_or_fragment_definition<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }

    /// Called after writing the operation name and/or arguments.
    #[inline]
    fn end_operation_signature<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }

    #[inline]
    fn begin_selection_set<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }

    /// Writes a `null` value to the specified writer.
    #[inline]
    fn write_null<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"null")
    }

    /// Writes a `true` or `false` value to the specified writer.
    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(if value { b"true" } else { b"false" })
    }

    /// Writes an integer value like `-123` to the specified writer.
    #[inline]
    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }

    /// Writes an integer value like `123` to the specified writer.
    #[inline]
    fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }

    /// Writes a floating point value like `-31.26e+12` to the specified writer.
    #[inline]
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format_finite(value);
        writer.write_all(s.as_bytes())
    }

    /// Writes an unsinged byte value like "255" to the specified writer.
    #[inline]
    fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }

    /// Writes the representation of a byte array. Formatters can choose whether
    /// to represent bytes as a JSON array of integers (the default), or some
    /// JSON string encoding like hex or base64.
    #[inline]
    fn write_byte_array<W>(&mut self, writer: &mut W, value: &[u8]) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.begin_array(writer)?;

        let mut bytes = value.iter().peekable();
        while let Some(byte) = bytes.next() {
            self.write_u8(writer, *byte)?;

            if bytes.peek().is_some() {
                writer.write_all(b",")?;
            }
        }

        self.end_array(writer)?;

        Ok(())
    }

    /// Writes a string fragment that doesn't need any escaping to the specified writer.
    #[inline]
    fn write_string_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(fragment.as_bytes())
    }

    /// Writes a `$` to the specified writer.
    ///
    /// This must be called before writing a variable name.
    #[inline]
    fn begin_variable<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"$")
    }

    /// Writes a `!` to the specified writer.
    ///
    /// This must be called after writing a variable type name.
    #[inline]
    fn write_non_null_type_indicator<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"!")
    }

    /// Writes a `@` to the specified writer.
    ///
    /// This must be called before writing a directive name.
    #[inline]
    fn begin_directive<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"@")
    }

    /// Writes a whitespace separator to the specified writer.
    ///
    /// This is used to separate different parts of the GraphQL query such
    /// as the operation type keyword from the operation name.
    #[inline]
    fn write_separator<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b" ")
    }

    /// Writes a `:` to the specified writer.
    fn write_name_value_separator<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b":")
    }

    /// Writes a `=` to the specified writer.
    ///
    /// This is used to separate the name of a variable from its default value.
    fn write_variable_default_value_separator<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"=")
    }

    /// Writes a `,` to the specified writer.
    fn write_item_separator<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b",")
    }

    #[inline]
    fn begin_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")
    }

    #[inline]
    fn end_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")
    }

    /// Writes a `(` to the specified writer.
    #[inline]
    fn begin_arguments<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"(")
    }

    /// Writes a `)` to the specified writer.
    #[inline]
    fn end_arguments<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b")")
    }

    /// Writes a `[` to the specified writer.
    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"[")
    }

    /// Writes a `]` to the specified writer.
    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"]")
    }

    /// Writes a `{` to the specified writer.
    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"{")
    }

    /// Writes a `}` to the specified writer.
    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"}")
    }

    /// Writes a `{` to the specified writer.
    #[inline]
    fn begin_block<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"{")
    }

    /// Writes a `}` to the specified writer.
    #[inline]
    fn end_block<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"}")
    }

    #[inline]
    fn begin_block_item<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }

    #[inline]
    fn end_block_item<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b",")
    }

    /// Writes a raw GraphQL fragment that doesn't need escaping to the writer.
    #[inline]
    fn write_raw<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(fragment.as_bytes())
    }
}
