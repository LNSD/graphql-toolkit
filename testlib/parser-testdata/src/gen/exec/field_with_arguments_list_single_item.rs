/// Test vector: `exec/field_with_arguments_list_single_item`
///
/// ```graphql
/// {
///   field(id: 1, argv: [1])
/// }
/// ```
///
/// See file: `exec/field_with_arguments_list_single_item.graphql`
pub const FIELD_WITH_ARGUMENTS_LIST_SINGLE_ITEM: &str = indoc::indoc! {r###"
  {
    field(id: 1, argv: [1])
  }
"###};
