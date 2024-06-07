/// Test vector: `exec/op_query_with_variables_with_default_value_null`
///
/// ```graphql
/// query($var: String = null) {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_variables_with_default_value_null.graphql`
pub const OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE_NULL: &str = indoc::indoc! {r###"
  query($var: String = null) {
    field
  }
"###};
