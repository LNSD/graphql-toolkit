/// Test vector: `exec/op_subscription_with_name`
///
/// ```graphql
/// subscription MySubscription {
///   field
/// }
/// ```
///
/// See file: `exec/op_subscription_with_name.graphql`
pub const OP_SUBSCRIPTION_WITH_NAME: &str = indoc::indoc! { r###"
  subscription MySubscription {
    field
  }"### };
