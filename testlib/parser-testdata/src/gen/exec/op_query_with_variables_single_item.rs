/// Test vector: `exec/op_query_with_variables_single_item`
///
/// ```graphql
/// query($var: Int) {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_variables_single_item.graphql`
pub const OP_QUERY_WITH_VARIABLES_SINGLE_ITEM: &str = indoc::indoc! {r###"
  query($var: Int) {
    field
  }
"###};
