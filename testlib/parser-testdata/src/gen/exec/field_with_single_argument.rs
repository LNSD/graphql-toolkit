/// Test vector: `exec/field_with_single_argument`
///
/// ```graphql
/// {
///   field(arg: 42)
/// }
/// ```
///
/// See file: `exec/field_with_single_argument.graphql`
pub const FIELD_WITH_SINGLE_ARGUMENT: &str = indoc::indoc! {r###"
  {
    field(arg: 42)
  }
"###};
