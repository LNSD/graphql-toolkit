/// Test vector: `exec/op_query_with_name_and_directives`
///
/// ```graphql
/// query MyQuery @directive {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_name_and_directives.graphql`
pub const OP_QUERY_WITH_NAME_AND_DIRECTIVES: &str = indoc::indoc! { r###"
  query MyQuery @directive {
    field
  }"### };
