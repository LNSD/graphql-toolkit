use super::formatter::Formatter;

/// This formatter compacts a GraphQL expression with no extra whitespace.
#[derive(Clone, Debug)]
pub struct CompactFormatter;

impl Formatter for CompactFormatter {}
