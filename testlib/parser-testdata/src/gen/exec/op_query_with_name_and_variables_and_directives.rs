/// Test vector: `exec/op_query_with_name_and_variables_and_directives`
///
/// ```graphql
/// query MyQuery($var: String = null) @directive {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_name_and_variables_and_directives.graphql`
pub const OP_QUERY_WITH_NAME_AND_VARIABLES_AND_DIRECTIVES: &str = indoc::indoc! { r###"
  query MyQuery($var: String = null) @directive {
    field
  }"### };
