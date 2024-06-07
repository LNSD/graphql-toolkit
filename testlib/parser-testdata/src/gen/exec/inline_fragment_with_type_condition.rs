/// Test vector: `exec/inline_fragment_with_type_condition`
///
/// ```graphql
/// {
///   ...on MyType {
///     field
///   }
/// }
/// ```
///
/// See file: `exec/inline_fragment_with_type_condition.graphql`
pub const INLINE_FRAGMENT_WITH_TYPE_CONDITION: &str = indoc::indoc! {r###"
  {
    ...on MyType {
      field
    }
  }
"###};
