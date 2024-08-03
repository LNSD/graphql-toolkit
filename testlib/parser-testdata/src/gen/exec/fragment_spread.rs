/// Test vector: `exec/fragment_spread`
///
/// ```graphql
/// {
///   ...MyFragment
/// }
/// ```
///
/// See file: `exec/fragment_spread.graphql`
pub const FRAGMENT_SPREAD: &str = indoc::indoc! { r###"
  {
    ...MyFragment
  }"### };
