/// Test vector: `exec/field_with_directives_with_arguments`
///
/// ```graphql
/// {
///   field @skip(if: $skip)
/// }
/// ```
///
/// See file: `exec/field_with_directives_with_arguments.graphql`
pub const FIELD_WITH_DIRECTIVES_WITH_ARGUMENTS: &str = indoc::indoc! {r###"
  {
    field @skip(if: $skip)
  }
"###};
