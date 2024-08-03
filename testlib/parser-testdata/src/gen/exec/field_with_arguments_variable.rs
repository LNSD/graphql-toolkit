/// Test vector: `exec/field_with_arguments_variable`
///
/// ```graphql
/// {
///   field(arg: $var)
/// }
/// ```
///
/// See file: `exec/field_with_arguments_variable.graphql`
pub const FIELD_WITH_ARGUMENTS_VARIABLE: &str = indoc::indoc! { r###"
  {
    field(arg: $var)
  }"### };
