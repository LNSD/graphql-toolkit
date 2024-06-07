/// Test vector: `exec/op_query_with_name`
///
/// ```graphql
/// query MyQuery{
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_name.graphql`
pub const OP_QUERY_WITH_NAME: &str = indoc::indoc! {r###"
  query MyQuery{
    field
  }
"###};
