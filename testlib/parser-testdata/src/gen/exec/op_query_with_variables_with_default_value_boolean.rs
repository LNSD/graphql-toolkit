/// Test vector: `exec/op_query_with_variables_with_default_value_boolean`
///
/// ```graphql
/// query($var: Boolean = true) {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_variables_with_default_value_boolean.graphql`
pub const OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE_BOOLEAN: &str = indoc::indoc! { r###"
  query($var: Boolean = true) {
    field
  }"### };
