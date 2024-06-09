/// Test vector: `exec/field_with_arguments_string_literal`
///
/// ```graphql
/// {
///   field(arg: "hello, world!")
/// }
/// ```
///
/// See file: `exec/field_with_arguments_string_literal.graphql`
pub const FIELD_WITH_ARGUMENTS_STRING_LITERAL: &str = indoc::indoc! {r###"
  {
    field(arg: "hello, world!")
  }
"###};
