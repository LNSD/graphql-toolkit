//! Serialize a Rust GraphQL AST data into a GraphQL document data.

use std::io;

use graphql_toolkit_ast::{
    ConstValue, Directive, ExecutableDocument, Field, FragmentDefinition, FragmentSpread,
    InlineFragment, Name, Number, OperationDefinition, Positioned, Selection, SelectionSet, Type,
    TypeCondition, Value, VariableDefinition,
};

pub use self::{
    compact_formatter::CompactFormatter, formatter::Formatter, pretty_formatter::PrettyFormatter,
    serializer::Serializer,
};

mod compact_formatter;
mod formatter;
mod pretty_formatter;
mod serializer;

/// A trait for serializing a Rust GraphQL AST data into a GraphQL document data.
pub trait AstSerialize {
    /// Serialize this value into the given serializer.
    fn serialize<W, F>(&self, ser: &mut Serializer<W, F>) -> anyhow::Result<()>
    where
        W: io::Write,
        F: Formatter;
}

// Implement `AstSerialize` for a type that can be serialized.
macro_rules! impl_serialize {
    ($ty:ty, $method:ident) => {
        impl AstSerialize for $ty {
            #[inline]
            fn serialize<W, F>(&self, ser: &mut Serializer<W, F>) -> anyhow::Result<()>
            where
                W: io::Write,
                F: Formatter,
            {
                ser.$method(self)
            }
        }
    };
}

/// Implement `AstSerialize` for the `Positioned` version of a type that can be serialized.
macro_rules! impl_serialize_positioned {
    ($ty:ty) => {
        impl AstSerialize for Positioned<$ty> {
            #[inline]
            fn serialize<W, F>(&self, ser: &mut Serializer<W, F>) -> anyhow::Result<()>
            where
                W: io::Write,
                F: Formatter,
            {
                self.node.serialize(ser)
            }
        }
    };
}

impl AstSerialize for OperationDefinition {
    #[inline]
    fn serialize<W, F>(&self, ser: &mut Serializer<W, F>) -> anyhow::Result<()>
    where
        W: io::Write,
        F: Formatter,
    {
        // If we are serializing an "operation definition" instance assume that there is a single
        // operation in the document
        ser.serialize_operation_definition(self, None, true)
    }
}

impl AstSerialize for (Name, OperationDefinition) {
    #[inline]
    fn serialize<W, F>(&self, ser: &mut Serializer<W, F>) -> anyhow::Result<()>
    where
        W: io::Write,
        F: Formatter,
    {
        let (name, operation) = self;
        ser.serialize_operation_definition(operation, Some(name), true)
    }
}

impl AstSerialize for (Name, FragmentDefinition) {
    #[inline]
    fn serialize<W, F>(&self, ser: &mut Serializer<W, F>) -> anyhow::Result<()>
    where
        W: io::Write,
        F: Formatter,
    {
        let (name, fragment) = self;
        ser.serialize_fragment_definition(name, fragment)
    }
}

impl_serialize!(ExecutableDocument, serialize_executable_document);
impl_serialize_positioned!(OperationDefinition);
impl_serialize!(SelectionSet, serialize_selection_set);
impl_serialize_positioned!(SelectionSet);
impl_serialize!(Selection, serialize_selection);
impl_serialize_positioned!(Selection);
impl_serialize!(Field, serialize_selection_field);
impl_serialize_positioned!(Field);
impl_serialize!(FragmentSpread, serialize_fragment_spread);
impl_serialize_positioned!(FragmentSpread);
impl_serialize!(InlineFragment, serialize_inline_fragment);
impl_serialize_positioned!(InlineFragment);
impl_serialize!(Directive, serialize_directive);
impl_serialize_positioned!(Directive);
impl_serialize!(VariableDefinition, serialize_variable_definition);
impl_serialize_positioned!(VariableDefinition);
impl_serialize!(Type, serialize_type);
impl_serialize_positioned!(Type);
impl_serialize!(ConstValue, serialize_const_value);
impl_serialize_positioned!(ConstValue);
impl_serialize!(TypeCondition, serialize_type_condition);
impl_serialize_positioned!(TypeCondition);
impl_serialize!(Name, serialize_name);
impl_serialize_positioned!(Name);
impl_serialize!(Value, serialize_value);
impl_serialize_positioned!(Value);
impl_serialize!(Number, serialize_number);

/// Serialize the given GraphQL AST as a compact GraphQL document into the I/O stream.
#[inline]
pub fn to_writer<W, T>(writer: W, value: &T) -> anyhow::Result<()>
where
    W: io::Write,
    T: ?Sized + AstSerialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}

/// Serialize the given GraphQL AST as a pretty-printed GraphQL document into the I/O stream.
#[inline]
pub fn to_writer_pretty<W, T>(writer: W, value: &T) -> anyhow::Result<()>
where
    W: io::Write,
    T: ?Sized + AstSerialize,
{
    let mut ser = Serializer::with_formatter(writer, PrettyFormatter::new());
    value.serialize(&mut ser)
}

/// Serialize the given GraphQL AST as a compact GraphQL document byte vector.
#[inline]
pub fn to_vec<T>(value: &T) -> anyhow::Result<Vec<u8>>
where
    T: ?Sized + AstSerialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given GraphQL AST as a pretty-printed GraphQL document byte vector.
#[inline]
pub fn to_vec_pretty<T>(value: &T) -> anyhow::Result<Vec<u8>>
where
    T: ?Sized + AstSerialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer_pretty(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given GraphQL AST as a compact GraphQL document string.
#[inline]
pub fn to_string<T>(value: &T) -> anyhow::Result<String>
where
    T: ?Sized + AstSerialize,
{
    let vec = to_vec(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

/// Serialize the given GraphQL AST as a pretty-printed GraphQL document string.
#[inline]
pub fn to_string_pretty<T>(value: &T) -> anyhow::Result<String>
where
    T: ?Sized + AstSerialize,
{
    let vec = to_vec_pretty(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}
