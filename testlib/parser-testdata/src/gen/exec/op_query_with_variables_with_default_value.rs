/// Test vector: `exec/op_query_with_variables_with_default_value`
///
/// ```graphql
/// query($var1: Int = 1, $var2: Int = 2) {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_variables_with_default_value.graphql`
pub const OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE: &str = indoc::indoc! {r###"
  query($var1: Int = 1, $var2: Int = 2) {
    field
  }
"###};
