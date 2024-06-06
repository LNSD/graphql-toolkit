use graphql_toolkit::ser::{AstSerialize, PrettyFormatter, Serializer};
use graphql_toolkit_ast::{
    AstPositionExt as _, BaseType, ConstValue, DocumentOperations, ExecutableDocument, Field, Name,
    OperationDefinition, OperationType, Selection, SelectionSet, Type, Value, VariableDefinition,
};

/// Test helper function to parse the input using async-graphql-parser.
///
/// # Panics
/// The function asserts that the input is a valarg GraphQL string by checking the result of the
/// `parse_query` function.
fn parse_query<I: AsRef<str>>(input: I) -> ExecutableDocument {
    graphql_toolkit::de::parse_query(input).expect("query parsing failed")
}

/// Test helper function to serialize the input using the pretty formatter.
fn to_string<T>(input: &T) -> anyhow::Result<String>
where
    T: ?Sized + AstSerialize,
{
    let mut buffer = Vec::with_capacity(128);
    let mut serializer = Serializer::with_formatter(&mut buffer, PrettyFormatter::new());

    input.serialize(&mut serializer)?;

    // Safety: The buffer is guaranteed to be a valarg UTF-8 string because the `CompactFormatter`
    // only writes valarg UTF-8 bytes.
    Ok(unsafe { String::from_utf8_unchecked(buffer) })
}

/// Test helper function to assert that the input is a valarg GraphQL string by checking the result
/// of the `parse_query` function.
fn assert_valid_query<I: AsRef<str>>(input: I) {
    graphql_toolkit::de::parse_query(input).expect("query parsing failed");
}

#[test]
fn query_shorthand() {
    //* Given
    let expected_query = parse_query(r#"{hello}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_named() {
    //* Given
    let expected_query = parse_query(r#"query MyQuery{field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
#[ignore = "Disabled due to non-deterministic output"]
fn query_multiple_operations() {
    //* Given
    let expected_query = parse_query(r#"query MyQuery{field1}query YourQuery{field2}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variables() {
    //* Given
    let expected_query = parse_query(r#"query($foo:String){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_named_with_variables() {
    //* Given
    let expected_query = parse_query(r#"query MyQuery($var1:String!){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variables_with_non_null_type() {
    //* Given
    let expected_query = parse_query(r#"query($var1:String!){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);

    // The query is valid, but the parser does not support non-null types.
    // assert_valid_query(&query);
}

#[test]
fn query_with_variables_with_type_list() {
    //* Given
    let expected_query = parse_query(r#"query($var1:[String]){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variables_with_type_list_non_null() {
    //* Given
    let expected_query = parse_query(r#"query($var1:[String]!){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variables_with_type_list_with_item_non_null() {
    //* Given
    let expected_query = parse_query(r#"query($var1:[String!]!){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variables_with_default_value_null() {
    //* Given
    let expected_query = parse_query(r#"query($var1:String=null){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variables_with_default_value_enum() {
    //* Given
    let expected_query = parse_query(r#"query($var1:SomeEnum=ACTIVE){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variables_with_default_value_bool() {
    //* Given
    let expected_query = parse_query(r#"query($var1:Bool=false){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variables_with_default_value_number() {
    //* Given
    let expected_query = parse_query(r#"query($var1:Float=1.61803){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variables_with_default_value_string() {
    //* Given
    let expected_query = parse_query(r#"query($var1:String="value1"){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variables_with_default_value_list() {
    //* Given
    let expected_query = parse_query(r#"query($var1:[Int]=[1,2,3]){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variables_with_default_value_binary_array() {
    //* Given
    // query(argv:Bytes=Bytes{1, 2, 3}){field}}
    let expected_query = ExecutableDocument {
        operations: DocumentOperations::Single(
            OperationDefinition {
                ty: OperationType::Query,
                variable_definitions: vec![VariableDefinition {
                    name: Name::new("argv").default_position(),
                    var_type: Type {
                        base: BaseType::Named(Name::new("Bytes")),
                        nullable: true,
                    }
                    .default_position(),
                    directives: vec![],
                    default_value: Some(
                        ConstValue::Binary(vec![1u8, 2u8, 3u8].into()).default_position(),
                    ),
                }
                .default_position()],
                directives: Default::default(),
                selection_set: SelectionSet {
                    items: vec![Selection::Field(
                        Field {
                            alias: None,
                            name: Name::new("field").default_position(),
                            arguments: vec![],
                            directives: vec![],
                            selection_set: Default::default(),
                        }
                        .default_position(),
                    )
                    .default_position()],
                }
                .default_position(),
            }
            .default_position(),
        ),
        fragments: Default::default(),
    };

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);

    // The query is valid, but the parser does not support binary arrays.
    // assert_valid_query(&query);
}

#[test]
fn query_with_variables_with_default_value_object() {
    //* Given
    let expected_query =
        parse_query(r#"query($var1:SomeInput={key1:"value",key2:1.61803}){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_directive() {
    //* Given
    let expected_query = parse_query(r#"query @include(if:true){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_named_with_directive() {
    //* Given
    let expected_query = parse_query(r#"query MyQuery@deprecated{field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_directive_with_arguments() {
    //* Given
    let expected_query = parse_query(r#"query @skip(if:true){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_directives_multiple() {
    //* Given
    let expected_query = parse_query(r#"query @skip(if:false)@defer{field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variables_and_directives() {
    //* Given
    let expected_query =
        parse_query(r#"query($var1:String!,$var2:Int)@skip(if:true)@defer{field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_named_with_variables_and_directives() {
    //* Given
    let expected_query =
        parse_query(r#"query MyQuery($var1:String!,$var2:Bool)@defer@skip(if:$var2){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
#[ignore = "Disabled due to parser bug"]
fn query_with_variables_with_default_value_and_directive() {
    //* Given
    let expected_query = parse_query(r#"query($var1:String="value1"@deprecated){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn mutation_unnamed() {
    //* Given
    let expected_query = parse_query(r#"mutation{dropTables}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn mutation_named() {
    //* Given
    let expected_query = parse_query(r#"mutation MyMutation{dropTables}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn subscription_unnamed() {
    //* Given
    let expected_query = parse_query(r#"subscription{newBlock}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn subscription_named() {
    //* Given
    let expected_query = parse_query(r#"subscription MySubscription{newBlock}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_multiple_fields() {
    //* Given
    let expected_query = parse_query(r#"{field1,field2}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_nested_fields() {
    //* Given
    let expected_query = parse_query(r#"{field1{field12{field13}},field2}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_alias() {
    //* Given
    let expected_query = parse_query(r#"{an_alias:field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_null_argument() {
    //* Given
    let expected_query = parse_query(r#"{field(arg1:null){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_enum_argument() {
    //* Given
    let expected_query = parse_query(r#"{field(arg1:ACTIVE){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_variable_argument() {
    //* Given
    let expected_query = parse_query(r#"{field(arg1:$var){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_boolean_argument() {
    //* Given
    let expected_query = parse_query(r#"{field(arg1:true){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_number_argument() {
    //* Given
    let expected_query = parse_query(r#"{field(arg1:-0.0){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_string_argument() {
    //* Given
    let expected_query = parse_query(r#"{field1(arg1:"value1"){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

// TODO: Add tests covering the multiline string literal argument serialization
//  https://github.com/async-graphql/async-graphql/blob/v5.0.0/parser/tests/executables/multiline_string.graphql
#[test]
#[ignore = "TODO"] // https://docs.rs/serde_json/latest/src/serde_json/ser.rs.html#2051
fn query_field_multiline_string_argument() {
    //* Given
    let expected_query = parse_query(indoc::indoc! {
        r#"{
            rust(arg: """
            My name
              is

            Ferris
            """) {
              field2
            }
        }"#
    });

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_multiple_arguments() {
    //* Given
    let expected_query = parse_query(r#"{field1(arg1:"value1",arg2:"value2"){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_multiple_number_arguments() {
    //* Given
    let expected_query = parse_query(
        r#"{field(arg1:-1,arg2:-0,arg3:0,arg4:-1.23,arg5:1.23,arg6:1.23e+2,arg7:0.123){field2}}"#,
    );

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_list_argument() {
    //* Given
    let expected_query = parse_query(r#"{field(argv:[1,2,3]){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_list_argument_nested() {
    //* Given
    let expected_query = parse_query(r#"{field(argv:[[1],[2,3],[4,5,6]]){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}
#[test]
fn query_with_field_and_bytes_argument() {
    //* Given
    // {field(argv: Bytes{1, 2, 3}){field2}}
    let expected_query = ExecutableDocument {
        operations: DocumentOperations::Single(
            OperationDefinition {
                ty: OperationType::Query,
                variable_definitions: Default::default(),
                directives: Default::default(),
                selection_set: SelectionSet {
                    items: vec![Selection::Field(
                        Field {
                            alias: None,
                            name: Name::new("field").default_position(),
                            arguments: vec![(
                                Name::new("argv").default_position(),
                                Value::Binary(vec![1u8, 2u8, 3u8].into()).default_position(),
                            )],
                            directives: vec![],
                            selection_set: SelectionSet {
                                items: vec![Selection::Field(
                                    Field {
                                        alias: None,
                                        name: Name::new("field2").default_position(),
                                        arguments: vec![],
                                        directives: vec![],
                                        selection_set: Default::default(),
                                    }
                                    .default_position(),
                                )
                                .default_position()],
                            }
                            .default_position(),
                        }
                        .default_position(),
                    )
                    .default_position()],
                }
                .default_position(),
            }
            .default_position(),
        ),
        fragments: Default::default(),
    };

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_object_argument() {
    //* Given
    let expected_query = parse_query(r#"{field(arg1:{key1:"value",key2:1.61803}){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_object_argument_nested() {
    //* Given
    let expected_query = parse_query(r#"{field(arg1:{key1:{key2:"value"}}){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_directive() {
    //* Given
    let expected_query = parse_query(r#"{field1@defer{field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_directive_with_arguments() {
    //* Given
    let expected_query = parse_query(r#"{field1@skip(if:$prod){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
}

#[test]
fn query_with_field_and_multiple_directives() {
    //* Given
    let expected_query = parse_query(r#"{field1@lowerCase@skip(if:true){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_arguments_and_directive() {
    //* Given
    let expected_query =
        parse_query(r#"{field1(arg1:"value1",arg2:"value2")@skip(if:true){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_spread_fragment() {
    //* Given
    let expected_query = parse_query(r#"{...fragmentName,field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_spread_fragment_with_directive() {
    //* Given
    let expected_query = parse_query(r#"{...fragmentName@defer,field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_inline_fragment() {
    //* Given
    let expected_query = parse_query(r#"{...{field21,field22},field1}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_inline_fragment_with_type_condition() {
    //* Given
    let expected_query = parse_query(r#"{...on SomeType{field21,field22},field1}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_inline_fragment_with_directive() {
    //* Given
    let expected_query = parse_query(r#"{...@include(if:$expandedInfo){field21,field22},field1}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_inline_fragment_with_multiple_directives() {
    //* Given
    let expected_query =
        parse_query(r#"{...@include(if:$expandedInfo)@deprecated{field21,field22},field1}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_inline_fragment_with_type_condition_and_directive() {
    //* Given
    let expected_query =
        parse_query(r#"{...on SomeType@include(if:$expandedInfo){field21,field22},field1}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_and_fragment() {
    //* Given
    let expected_query = parse_query(r#"{field1}fragment MyFragment on SomeType{field2}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
}

#[test]
fn query_and_fragment_with_directive() {
    //* Given
    let expected_query = parse_query(r#"{field1}fragment MyFragment on SomeType@defer{field2}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
}

#[test]
#[ignore = "Disabled due to non-deterministic output"]
fn query_and_fragments_multiple() {
    //* Given
    let expected_query = parse_query(
        r#"fragment MyFragment on SomeType{field2}fragment YourFragment on AnotherType{field3}{field1}"#,
    );

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
}
