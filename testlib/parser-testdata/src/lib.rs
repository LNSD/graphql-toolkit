//! Test vectors for GraphQL parser.

/// Test vectors set containing test vectors to test the parsing of GraphQL executable documents.
pub mod exec {
    include!("gen/exec/mod.rs");
}

/// Test vectors set containing the official GraphQL "kitchen sink" test vectors.
///
/// These test vectors are designed to perform a sanity check on the parser implementation.
pub mod kitchen_sink {
    include!("gen/kitchen-sink/mod.rs");
}
