/// Test vector: `exec/document_with_op_query_and_fragments`
///
/// ```graphql
/// query {
///   field
/// }
///
/// fragment MyFragment on MyType {
///   field
/// }
/// ```
///
/// See file: `exec/document_with_op_query_and_fragments.graphql`
pub const DOCUMENT_WITH_OP_QUERY_AND_FRAGMENTS: &str = indoc::indoc! { r###"
  query {
    field
  }

  fragment MyFragment on MyType {
    field
  }"### };
