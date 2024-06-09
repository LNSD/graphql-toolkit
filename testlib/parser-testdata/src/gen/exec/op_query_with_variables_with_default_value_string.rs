/// Test vector: `exec/op_query_with_variables_with_default_value_string`
///
/// ```graphql
/// query($var1: Int, $var2: String = "value") {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_variables_with_default_value_string.graphql`
pub const OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE_STRING: &str = indoc::indoc! {r###"
  query($var1: Int, $var2: String = "value") {
    field
  }
"###};
