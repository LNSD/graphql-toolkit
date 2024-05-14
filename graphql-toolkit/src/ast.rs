//! This module contains the AST types for the GraphQL query language.

pub use async_graphql_parser::{types::*, Pos, Positioned};
pub use async_graphql_value::*;

/// AST extension trait for adding position information to AST nodes.
pub trait AstPositionExt {
    /// Create a non-positioned version of this AST node.
    fn default_position(self) -> Positioned<Self>;

    /// Create a positioned version of this AST node.
    fn with_position(self, pos: Pos) -> Positioned<Self>;
}

macro_rules! impl_ast_position_ext {
    ($($ty:ty),* $(,)?) => {
        $(
            impl AstPositionExt for $ty {
                fn default_position(self) -> Positioned<Self> {
                    Positioned::new(self, Default::default())
                }

                fn with_position(self, pos: Pos) -> Positioned<Self> {
                    Positioned::new(self, pos)
                }
            }
        )*
    };
    () => {};
}

// Alphabetically sorted list of AST types
impl_ast_position_ext! {
    BaseType, ConstValue, Directive, DocumentOperations, Field, FragmentDefinition,
    InlineFragment, Name, Number, OperationDefinition, OperationType, Selection, SelectionSet,
    Type, TypeCondition, Value, VariableDefinition,
}
