/// Test vector: `exec/document_multiple_op_query_with_name`
///
/// ```graphql
/// query First{
///   field1
/// }
///
/// query Second{
///   field2
/// }
/// ```
///
/// See file: `exec/document_multiple_op_query_with_name.graphql`
pub const DOCUMENT_MULTIPLE_OP_QUERY_WITH_NAME: &str = indoc::indoc! {r###"
  query First{
    field1
  }

  query Second{
    field2
  }
"###};
