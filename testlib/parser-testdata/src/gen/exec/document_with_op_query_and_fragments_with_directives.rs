/// Test vector: `exec/document_with_op_query_and_fragments_with_directives`
///
/// ```graphql
/// query {
///   field
/// }
///
/// fragment MyFragment on MyType @directive {
///   field
/// }
/// ```
///
/// See file: `exec/document_with_op_query_and_fragments_with_directives.graphql`
pub const DOCUMENT_WITH_OP_QUERY_AND_FRAGMENTS_WITH_DIRECTIVES: &str = indoc::indoc! { r###"
  query {
    field
  }

  fragment MyFragment on MyType @directive {
    field
  }"### };
