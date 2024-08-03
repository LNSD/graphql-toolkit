/// Test vector: `exec/field_with_arguments_list_nested`
///
/// ```graphql
/// {
///   field(argv: [[1, 2], [3, 4]], kwargs: null)
/// }
/// ```
///
/// See file: `exec/field_with_arguments_list_nested.graphql`
pub const FIELD_WITH_ARGUMENTS_LIST_NESTED: &str = indoc::indoc! { r###"
  {
    field(argv: [[1, 2], [3, 4]], kwargs: null)
  }"### };
