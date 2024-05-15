use super::formatter::Formatter;

/// This formatter generates a compact GraphQL document with no extra whitespace.
#[derive(Clone, Debug)]
pub struct CompactFormatter;

impl Formatter for CompactFormatter {}
