/// Test vector: `exec/op_query_with_variables_with_default_value_enum`
///
/// ```graphql
/// query($var: Status = ENABLED) {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_variables_with_default_value_enum.graphql`
pub const OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE_ENUM: &str = indoc::indoc! {r###"
  query($var: Status = ENABLED) {
    field
  }
"###};
