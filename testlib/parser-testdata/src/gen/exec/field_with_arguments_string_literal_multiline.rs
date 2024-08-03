/// Test vector: `exec/field_with_arguments_string_literal_multiline`
///
/// ```graphql
/// {
///   field(arg: """
///   My name
///     is
///
///   Ferris
///   """)
/// }
/// ```
///
/// See file: `exec/field_with_arguments_string_literal_multiline.graphql`
pub const FIELD_WITH_ARGUMENTS_STRING_LITERAL_MULTILINE: &str = indoc::indoc! { r###"
  {
    field(arg: """
    My name
      is

    Ferris
    """)
  }"### };
