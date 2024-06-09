/// Test vector: `exec/field_with_directives`
///
/// ```graphql
/// {
///   field @dir1 @dir2
/// }
/// ```
///
/// See file: `exec/field_with_directives.graphql`
pub const FIELD_WITH_DIRECTIVES: &str = indoc::indoc! {r###"
  {
    field @dir1 @dir2
  }
"###};
