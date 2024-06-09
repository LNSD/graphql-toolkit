/// Test vector: `exec/op_query_with_variables`
///
/// ```graphql
/// query($var1: Int, $var2: String) {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_variables.graphql`
pub const OP_QUERY_WITH_VARIABLES: &str = indoc::indoc! {r###"
  query($var1: Int, $var2: String) {
    field
  }
"###};
