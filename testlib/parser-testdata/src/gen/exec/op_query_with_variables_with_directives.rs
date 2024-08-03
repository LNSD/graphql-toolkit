/// Test vector: `exec/op_query_with_variables_with_directives`
///
/// ```graphql
/// query($var: Int @directive) {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_variables_with_directives.graphql`
pub const OP_QUERY_WITH_VARIABLES_WITH_DIRECTIVES: &str = indoc::indoc! { r###"
  query($var: Int @directive) {
    field
  }"### };
