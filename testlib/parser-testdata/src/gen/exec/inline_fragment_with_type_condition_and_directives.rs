/// Test vector: `exec/inline_fragment_with_type_condition_and_directives`
///
/// ```graphql
/// {
///   ...on MyType @directive {
///     field
///   }
/// }
/// ```
///
/// See file: `exec/inline_fragment_with_type_condition_and_directives.graphql`
pub const INLINE_FRAGMENT_WITH_TYPE_CONDITION_AND_DIRECTIVES: &str = indoc::indoc! { r###"
  {
    ...on MyType @directive {
      field
    }
  }"### };
