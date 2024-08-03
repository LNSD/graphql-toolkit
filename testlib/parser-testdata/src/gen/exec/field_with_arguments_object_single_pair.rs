/// Test vector: `exec/field_with_arguments_object_single_pair`
///
/// ```graphql
/// {
///   field(kwargs: {key: "value"})
/// }
/// ```
///
/// See file: `exec/field_with_arguments_object_single_pair.graphql`
pub const FIELD_WITH_ARGUMENTS_OBJECT_SINGLE_PAIR: &str = indoc::indoc! { r###"
  {
    field(kwargs: {key: "value"})
  }"### };
