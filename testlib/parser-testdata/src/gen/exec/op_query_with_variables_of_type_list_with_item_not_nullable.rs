/// Test vector: `exec/op_query_with_variables_of_type_list_with_item_not_nullable`
///
/// ```graphql
/// query($var: [String!]) {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_variables_of_type_list_with_item_not_nullable.graphql`
pub const OP_QUERY_WITH_VARIABLES_OF_TYPE_LIST_WITH_ITEM_NOT_NULLABLE: &str = indoc::indoc! {r###"
  query($var: [String!]) {
    field
  }
"###};
