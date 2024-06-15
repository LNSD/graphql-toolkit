#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let _ = graphql_toolkit_parser::parse_query(data);
});
