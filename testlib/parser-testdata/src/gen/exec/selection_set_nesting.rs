/// Test vector: `exec/selection_set_nesting`
///
/// ```graphql
/// {
///   field1 {
///     field21
///     field22 {
///       field3
///     }
///   }
/// }
/// ```
///
/// See file: `exec/selection_set_nesting.graphql`
pub const SELECTION_SET_NESTING: &str = indoc::indoc! { r###"
  {
    field1 {
      field21
      field22 {
        field3
      }
    }
  }"### };
