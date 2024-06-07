/// Test vector: `exec/op_query_with_variables_with_default_value_list`
///
/// ```graphql
/// query($var1: [Int] = [1, 2, 3], $var2: Int, $var3: Float) {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_variables_with_default_value_list.graphql`
pub const OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE_LIST: &str = indoc::indoc! {r###"
  query($var1: [Int] = [1, 2, 3], $var2: Int, $var3: Float) {
    field
  }
"###};
