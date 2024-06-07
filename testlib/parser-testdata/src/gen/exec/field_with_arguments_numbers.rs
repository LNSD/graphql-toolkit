/// Test vector: `exec/field_with_arguments_numbers`
///
/// ```graphql
/// {
///   field(id: -1, id1: -0, id2: 0, id3: -1.23, id4:1.23, id5: 1.23e+2, id6: 0.123)
/// }
/// ```
///
/// See file: `exec/field_with_arguments_numbers.graphql`
pub const FIELD_WITH_ARGUMENTS_NUMBERS: &str = indoc::indoc! {r###"
  {
    field(id: -1, id1: -0, id2: 0, id3: -1.23, id4:1.23, id5: 1.23e+2, id6: 0.123)
  }
"###};
