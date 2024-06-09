/// Test vector: `exec/op_mutation_with_name`
///
/// ```graphql
/// mutation MyMutation{
///   field
/// }
/// ```
///
/// See file: `exec/op_mutation_with_name.graphql`
pub const OP_MUTATION_WITH_NAME: &str = indoc::indoc! {r###"
  mutation MyMutation{
    field
  }
"###};
