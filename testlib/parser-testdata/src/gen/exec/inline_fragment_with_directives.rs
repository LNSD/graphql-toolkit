/// Test vector: `exec/inline_fragment_with_directives`
///
/// ```graphql
/// {
///   ... @directive {
///     field
///   }
/// }
/// ```
///
/// See file: `exec/inline_fragment_with_directives.graphql`
pub const INLINE_FRAGMENT_WITH_DIRECTIVES: &str = indoc::indoc! {r###"
  {
    ... @directive {
      field
    }
  }
"###};
