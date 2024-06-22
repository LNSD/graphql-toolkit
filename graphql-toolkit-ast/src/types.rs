//! GraphQL types.
//!
//! The two root types are
//! [`ExecutableDocument`](struct.ExecutableDocument.html) and
//! [`ServiceDocument`](struct.ServiceDocument.html), representing an executable
//! GraphQL query and a GraphQL service respectively.
//!
//! This follows the [June 2018 edition of the GraphQL spec](https://spec.graphql.org/October2021/).

pub use common::*;
pub use executable::*;
pub use service::*;

mod common;
mod executable;
mod service;
