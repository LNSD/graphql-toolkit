use assert_matches::assert_matches;
use graphql_toolkit::{
    ast::{
        AstPositionExt as _, BaseType, ConstValue, DocumentOperations, ExecutableDocument, Field,
        Name, OperationDefinition, OperationType, Positioned, Selection, SelectionSet, Type, Value,
        VariableDefinition,
    },
    ser::{AstSerialize, CompactFormatter, Serializer},
};

/// Test helper function to parse the input using async-graphql-parser.
///
/// # Panics
/// The function asserts that the input is a valid GraphQL string by checking the result of the
/// `parse_query` function.
fn parse_query<I: AsRef<str>>(input: I) -> ExecutableDocument {
    graphql_toolkit::de::parse_query(input).expect("query parsing failed")
}

/// Test helper function to serialize the input using the compact formatter.
fn to_string<T>(input: &T) -> anyhow::Result<String>
where
    T: ?Sized + AstSerialize,
{
    let mut buffer = Vec::with_capacity(128);
    let mut serializer = Serializer::with_formatter(&mut buffer, CompactFormatter);

    input.serialize(&mut serializer)?;

    // Safety: The buffer is guaranteed to be a valid UTF-8 string because the `CompactFormatter`
    // only writes valid UTF-8 bytes.
    Ok(unsafe { String::from_utf8_unchecked(buffer) })
}

/// Test helper function to assert that the input is a valid GraphQL string by checking the result
/// of the `parse_query` function.
fn assert_valid_query<I: AsRef<str>>(input: I) {
    graphql_toolkit::de::parse_query(input).expect("query parsing failed");
}

#[test]
fn query_minimal() {
    //* Given
    let expected_query = parse_query(r#"{hello}"#);

    // TODO: Cover this case in another test
    // When serializing an [`OperationDefinition`] instance (or its "positioned" counterpart),
    // the operation type should be serialized as an anonymous single query.
    //
    // let query = OperationDefinition {
    //     ty: OperationType::Query,
    //     variable_definitions: vec![],
    //     directives: vec![],
    //     selection_set: SelectionSet {
    //         items: vec![Selection::Field(
    //             Field {
    //                 alias: None,
    //                 name: Name::new("hello").default_position(),
    //                 arguments: vec![],
    //                 directives: vec![],
    //                 selection_set: Default::default(),
    //             }
    //             .default_position(),
    //         )
    //         .default_position()],
    //     }
    //     .default_position(),
    // };

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
// TODO: Make the parser deterministic by replacing hashmaps with indexmap
#[ignore = "Non deterministic because of hashmap"]
fn query_multiple_named_queries() {
    //* Given
    let expected_query = parse_query(r#"query First{field1}query Second{field2}"#);

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
fn query_named_field_selections_nested() {
    //* Given
    let expected_query = parse_query(r#"query MyQuery{field1{field2{field3}}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_named_with_field_alias() {
    //* Given
    let expected_query = parse_query(r#"query MyQuery{an_alias:field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_field_single_argument() {
    //* Given
    let expected_query = parse_query(r#"{field(arg:42)}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_field_multiple_number_arguments() {
    //* Given
    let expected_query =
        parse_query(r#"{field(id:-1,id1:-0,id2:0,id3:-1.23,id4:1.23,id5:1.23e+2,id6:0.123)}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_field_string_literal_argument() {
    //* Given
    let expected_query = parse_query(r#"{field(arg:"hello, world!")}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

// TODO: Add tests covering the multiline string literal argument serialization
//  https://github.com/async-graphql/async-graphql/blob/v5.0.0/parser/tests/executables/multiline_string.graphql
#[test]
#[ignore] // https://docs.rs/serde_json/latest/src/serde_json/ser.rs.html#2051
fn query_field_multiline_string_literal_argument() {
    //* Given
    let expected_query = parse_query(indoc::indoc! {
        r#"{
            rust(arg: """
            My name
              is

            Ferris
            """)
        }"#
    });

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_field_boolean_literal_argument() {
    //* Given
    let expected_query = parse_query(r#"{field(arg:true){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_field_with_enum_argument() {
    //* Given
    let expected_query = parse_query(r#"{field(arg:ACTIVE){field2}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_field_list_argument() {
    //* Given
    let expected_query = parse_query(r#"{field(id:1,argv:[1,2,3])}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_field_list_argument_nested() {
    //* Given
    let expected_query = parse_query(r#"{field(id:1,argv:[[1,2],[3,4]])}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_field_bytes_argument() {
    //* Given
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
                                Name::new("arg").default_position(),
                                Value::Binary(vec![1u8, 2u8, 3u8].into()).default_position(),
                            )],
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
    assert_valid_query(&query);
}

#[test]
fn query_field_object_argument() {
    //* Given
    let expected_query = parse_query(r#"{field(arg:{key1:"value",key2:1.61803})}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_field_object_argument_nested() {
    //* Given
    let expected_query = parse_query(r#"{field(arg:{key1:{key2:"value"}})}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_field_with_directive() {
    //* Given
    let expected_query = parse_query(r#"{field@defer{count}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_field_with_directive_args() {
    //* Given
    let expected_query = parse_query(r#"{field@skip(if:$foo,nullable:null)}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_field_with_multiple_directives() {
    //* Given
    let expected_query = parse_query(r#"{field@defer@skip(if:$foo){count}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_directive() {
    //* Given
    let expected_query = parse_query(r#"query@dir{field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_directive_args() {
    //* Given
    let expected_query = parse_query(r#"query@dir(if:$foo){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_multiple_directives() {
    //* Given
    let expected_query = parse_query(r#"query@skip(if:$is_none)@defer{field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_named_with_directive() {
    //* Given
    let expected_query = parse_query(r#"query MyQuery@dir{field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variables() {
    //* Given
    let expected_query = parse_query(r#"query($foo:String!){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_multiple_variables() {
    //* Given
    let expected_query = parse_query(r#"query($foo:String!,$bar:Int){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_non_nullable_variable_type_nullable_items_list() {
    //* Given
    let expected_query = parse_query(r#"query($foo:[String]!){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_nullable_variable_type_non_null_items_list() {
    //* Given
    let expected_query = parse_query(r#"query($foo:[String!]){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variable_default_value_string_literal() {
    //* Given
    let expected_query = parse_query(r#"query($foo:String!="bar"){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variable_default_value_number_list_literal() {
    //* Given
    let expected_query = parse_query(r#"query($foo:[Int!]=[1,2,3]){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variable_default_value_object() {
    //* Given
    let expected_query = parse_query(r#"query($foo:KeyMap={key1:"value",key2:false}){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variable_default_value_enum() {
    //* Given
    let expected_query = parse_query(r#"query($foo:Status!=ENABLED){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variable_default_value_bytes() {
    //* Given
    let expected_query = ExecutableDocument {
        operations: DocumentOperations::Single(
            OperationDefinition {
                ty: OperationType::Query,
                variable_definitions: vec![VariableDefinition {
                    name: Name::new("foo").default_position(),
                    var_type: Type {
                        base: BaseType::Named(Name::new("Bytes")),
                        nullable: true,
                    }
                    .default_position(),
                    directives: vec![],
                    default_value: Some(
                        ConstValue::Binary(vec![0x01, 0x02, 0x03].into()).default_position(),
                    ),
                }
                .default_position()],
                directives: vec![],
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
    assert_valid_query(&query);

    let ast = parse_query(&query);
    assert_matches!(&ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { variable_definitions, ..}, ..}),
        ..
    } => {
        assert_eq!(variable_definitions.len(), 1);
        assert_matches!(&variable_definitions[0], Positioned { node: variable_definition, ..} => {
            assert_matches!(&variable_definition.default_value, Some(Positioned { node: value, ..}) => {
                assert_matches!(&value, ConstValue::List(list_values) => {
                    assert_matches!(&list_values[0], ConstValue::Number(num) => {
                        assert_matches!(num.as_u64(), Some(1));
                    });
                    assert_matches!(&list_values[1], ConstValue::Number(num) => {
                        assert_matches!(num.as_u64(), Some(2));
                    });
                    assert_matches!(&list_values[2], ConstValue::Number(num) => {
                        assert_matches!(num.as_u64(), Some(3));
                    });
                });
            });
        });
    });
}

#[test]
fn query_with_variable_and_directive() {
    //* Given
    let expected_query = parse_query(r#"query($foo:String@deprecated){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_variable_and_multiple_directives() {
    //* Given
    let expected_query = parse_query(r#"query($foo:String@lowerCase@deprecated){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_multiple_variables_and_directives() {
    //* Given
    let expected_query =
        parse_query(r#"query($foo:String!@lowerCase,$bar:String!@upperCase@deprecated){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
// TODO: Report bug to the async-graphql project
#[ignore = "Bug in the parser"]
fn query_with_variable_default_value_and_directive() {
    //* Given
    let expected_query = parse_query(r#"query($foo:String="bar"@deprecated){field}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_and_fragment() {
    //* Given
    let expected_query = parse_query(
        r#"query{field,...MyFragment}fragment MyFragment on MyFragmentTypeName{field}"#,
    );

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_and_fragment_with_directives() {
    //* Given
    let expected_query = parse_query(
        r#"query{field@defer,...MyFragment@skip(if:$foo)}fragment MyFragment on MyFragmentTypeName@directive{field}"#,
    );

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_inline_fragment() {
    //* Given
    let expected_query = parse_query(r#"{field,...on MyFragmentTypeName{field}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn query_with_field_and_inline_fragment_no_type_condition() {
    //* Given
    let expected_query = parse_query(r#"{field,...{field}}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn mutation_unnamed() {
    //* Given
    let expected_query = parse_query(r#"mutation{dropTable}"#);

    //* When
    let query = to_string(&expected_query).expect("Failed to serialize query");

    //* Then
    insta::assert_snapshot!(query);
    assert_valid_query(&query);
}

#[test]
fn mutation_named() {
    //* Given
    let expected_query = parse_query(r#"mutation MyMutation{dropTable}"#);

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
