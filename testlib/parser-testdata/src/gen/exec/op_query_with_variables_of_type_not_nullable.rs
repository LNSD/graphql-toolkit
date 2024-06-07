/// Test vector: `exec/op_query_with_variables_of_type_not_nullable`
///
/// ```graphql
/// query($var1: Int, $var2: String!) {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_variables_of_type_not_nullable.graphql`
pub const OP_QUERY_WITH_VARIABLES_OF_TYPE_NOT_NULLABLE: &str = indoc::indoc! {r###"
  query($var1: Int, $var2: String!) {
    field
  }
"###};
