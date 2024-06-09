/// Test vector: `exec/op_query_with_variables_with_default_value_object`
///
/// ```graphql
/// query($var1: KeyMap = {key1: "value1", key2: "value2"}) {
///   field
/// }
/// ```
///
/// See file: `exec/op_query_with_variables_with_default_value_object.graphql`
pub const OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE_OBJECT: &str = indoc::indoc! {r###"
  query($var1: KeyMap = {key1: "value1", key2: "value2"}) {
    field
  }
"###};
