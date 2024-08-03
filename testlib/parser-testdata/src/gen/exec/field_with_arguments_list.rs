/// Test vector: `exec/field_with_arguments_list`
///
/// ```graphql
/// {
///   field(id: 1, argv: [1, 2, 3])
/// }
/// ```
///
/// See file: `exec/field_with_arguments_list.graphql`
pub const FIELD_WITH_ARGUMENTS_LIST: &str = indoc::indoc! { r###"
  {
    field(id: 1, argv: [1, 2, 3])
  }"### };
