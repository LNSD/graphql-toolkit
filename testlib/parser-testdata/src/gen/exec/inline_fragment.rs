/// Test vector: `exec/inline_fragment`
///
/// ```graphql
/// {
///   ...{
///     field
///   }
/// }
/// ```
///
/// See file: `exec/inline_fragment.graphql`
pub const INLINE_FRAGMENT: &str = indoc::indoc! { r###"
  {
    ...{
      field
    }
  }"### };
