/// Test vector: `exec/op_query_with_variables_of_type_list`
///
/// ```graphql
/// query($var: [String]) {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_variables_of_type_list.graphql`
pub const OP_QUERY_WITH_VARIABLES_OF_TYPE_LIST: &str = indoc::indoc! { r###"
  query($var: [String]) {
    field
  }"### };
