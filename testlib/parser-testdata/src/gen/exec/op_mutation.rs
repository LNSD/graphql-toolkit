/// Test vector: `exec/op_mutation`
///
/// ```graphql
/// mutation {
///   field
/// }
/// ```
///
/// See file: `exec/op_mutation.graphql`
pub const OP_MUTATION: &str = indoc::indoc! {r###"
  mutation {
    field
  }
"###};
