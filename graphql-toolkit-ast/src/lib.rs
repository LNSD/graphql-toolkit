//! A collection of Rust modules to process GraphQL documents
//!
//! This module contains the AST types and traits for the GraphQL query language.

pub use graphql_toolkit_value::*;
pub use pos::*;
pub use types::*;

mod pos;
mod types;

/// A value-to-AST conversion trait that consumes the input value.
pub trait IntoAst<T> {
    /// Convert this value into an AST type.
    #[must_use]
    fn into_ast(self) -> T;
}

macro_rules! impl_into_ast {
    ($($ty:ty),* $(,)?) => {
        $(
            impl IntoAst<$ty> for $ty {
                /// Returns the input value unchanged.
                #[inline(always)]
                fn into_ast(self) -> $ty {
                    self
                }
            }
        )*
    };
}

impl<S> IntoAst<Name> for S
where
    S: AsRef<str>,
{
    #[inline]
    fn into_ast(self) -> Name {
        Name::new(self)
    }
}

// Alphabetically sorted list of AST types
impl_into_ast! {
    BaseType, ConstValue, Directive, DocumentOperations, Field, FragmentDefinition, FragmentSpread,
    InlineFragment, Number, OperationDefinition, OperationType, Selection, SelectionSet,
    Type, TypeCondition, Value, VariableDefinition,
}

/// Extension trait for adding position information to AST nodes.
pub trait AstPositionExt: Sized {
    /// Create a positioned version of this AST node with the default position (0:0).
    #[must_use]
    #[inline]
    fn default_position(self) -> Positioned<Self> {
        Positioned::new(self, Default::default())
    }

    /// Create a positioned version of this AST node with the given position.
    #[must_use]
    #[inline]
    fn with_position(self, pos: Pos) -> Positioned<Self> {
        Positioned::new(self, pos)
    }
}

macro_rules! impl_ast_position_ext {
    ($($ty:ty),* $(,)?) => {
        $(
            impl AstPositionExt for $ty {}
        )*
    };
}

// Alphabetically sorted list of AST types
impl_ast_position_ext! {
    BaseType, ConstValue, Directive, DocumentOperations, Field, FragmentDefinition, FragmentSpread,
    InlineFragment, Name, Number, OperationDefinition, OperationType, Selection, SelectionSet,
    Type, TypeCondition, Value, VariableDefinition,
}
