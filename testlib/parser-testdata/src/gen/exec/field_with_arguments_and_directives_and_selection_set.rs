/// Test vector: `exec/field_with_arguments_and_directives_and_selection_set`
///
/// ```graphql
/// {
///   field1(id: 42) @skip(if: $skip) {
///     field2
///   }
/// }
/// ```
///
/// See file: `exec/field_with_arguments_and_directives_and_selection_set.graphql`
pub const FIELD_WITH_ARGUMENTS_AND_DIRECTIVES_AND_SELECTION_SET: &str = indoc::indoc! {r###"
  {
    field1(id: 42) @skip(if: $skip) {
      field2
    }
  }
"###};
