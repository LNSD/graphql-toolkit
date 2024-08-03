/// Test vector: `exec/field_with_arguments_object_nested`
///
/// ```graphql
/// {
///   field(kwargs: {key11: {key2: "value2"}, key12: "value12"}, meta: null)
/// }
/// ```
///
/// See file: `exec/field_with_arguments_object_nested.graphql`
pub const FIELD_WITH_ARGUMENTS_OBJECT_NESTED: &str = indoc::indoc! { r###"
  {
    field(kwargs: {key11: {key2: "value2"}, key12: "value12"}, meta: null)
  }"### };
