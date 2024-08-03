/// Test vector: `exec/op_subscription`
///
/// ```graphql
/// subscription {
///   field
/// }
/// ```
///
/// See file: `exec/op_subscription.graphql`
pub const OP_SUBSCRIPTION: &str = indoc::indoc! { r###"
  subscription {
    field
  }"### };
