/// Test vector: `exec/fragment_spread_with_directives`
///
/// ```graphql
/// {
///   ...MyFragment @directive
/// }
/// ```
///
/// See file: `exec/fragment_spread_with_directives.graphql`
pub const FRAGMENT_SPREAD_WITH_DIRECTIVES: &str = indoc::indoc! { r###"
  {
    ...MyFragment @directive
  }"### };
