/// Test vector: `exec/field_with_arguments_object`
///
/// ```graphql
/// {
///   field(id: 1, kwargs: {key1: "value", key2: 1.61803})
/// }
/// ```
///
/// See file: `exec/field_with_arguments_object.graphql`
pub const FIELD_WITH_ARGUMENTS_OBJECT: &str = indoc::indoc! { r###"
  {
    field(id: 1, kwargs: {key1: "value", key2: 1.61803})
  }"### };
