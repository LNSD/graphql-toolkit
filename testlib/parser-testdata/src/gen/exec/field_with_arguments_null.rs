/// Test vector: `exec/field_with_arguments_null`
///
/// ```graphql
/// {
///   field(arg: null)
/// }
/// ```
///
/// See file: `exec/field_with_arguments_null.graphql`
pub const FIELD_WITH_ARGUMENTS_NULL: &str = indoc::indoc! { r###"
  {
    field(arg: null)
  }"### };
