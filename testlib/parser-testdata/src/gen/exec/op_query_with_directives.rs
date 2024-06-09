/// Test vector: `exec/op_query_with_directives`
///
/// ```graphql
/// query @dir1 @dir2 @dir3{
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_directives.graphql`
pub const OP_QUERY_WITH_DIRECTIVES: &str = indoc::indoc! {r###"
  query @dir1 @dir2 @dir3{
    field
  }
"###};
