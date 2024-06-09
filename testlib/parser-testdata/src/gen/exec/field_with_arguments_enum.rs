/// Test vector: `exec/field_with_arguments_enum`
///
/// ```graphql
/// {
///   field(arg: ACTIVE)
/// }
/// ```
///
/// See file: `exec/field_with_arguments_enum.graphql`
pub const FIELD_WITH_ARGUMENTS_ENUM: &str = indoc::indoc! {r###"
  {
    field(arg: ACTIVE)
  }
"###};
