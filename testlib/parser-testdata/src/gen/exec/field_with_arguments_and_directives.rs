/// Test vector: `exec/field_with_arguments_and_directives`
///
/// ```graphql
/// {
///   field(id: 42) @skip(if: $skip)
/// }
/// ```
///
/// See file: `exec/field_with_arguments_and_directives.graphql`
pub const FIELD_WITH_ARGUMENTS_AND_DIRECTIVES: &str = indoc::indoc! {r###"
  {
    field(id: 42) @skip(if: $skip)
  }
"###};
