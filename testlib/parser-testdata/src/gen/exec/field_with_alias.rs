/// Test vector: `exec/field_with_alias`
///
/// ```graphql
/// {
///   field_alias: field
/// }
/// ```
///
/// See file: `exec/field_with_alias.graphql`
pub const FIELD_WITH_ALIAS: &str = indoc::indoc! {r###"
  {
    field_alias: field
  }
"###};
