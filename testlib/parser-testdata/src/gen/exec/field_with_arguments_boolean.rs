/// Test vector: `exec/field_with_arguments_boolean`
///
/// ```graphql
/// {
///   field(arg: true)
/// }
/// ```
///
/// See file: `exec/field_with_arguments_boolean.graphql`
pub const FIELD_WITH_ARGUMENTS_BOOLEAN: &str = indoc::indoc! { r###"
  {
    field(arg: true)
  }"### };
