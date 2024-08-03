/// Test vector: `exec/selection_set_with_multiple_selections`
///
/// ```graphql
/// {
///   ...MyFragment
///   ... on MyType {
///     field2
///   }
///   field
/// }
/// ```
///
/// See file: `exec/selection_set_with_multiple_selections.graphql`
pub const SELECTION_SET_WITH_MULTIPLE_SELECTIONS: &str = indoc::indoc! { r###"
  {
    ...MyFragment
    ... on MyType {
      field2
    }
    field
  }"### };
