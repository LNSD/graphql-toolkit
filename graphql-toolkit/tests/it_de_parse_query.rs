use assert_matches::assert_matches;
use graphql_toolkit::de::parse_query;
use graphql_toolkit_ast::{
    BaseType, ConstValue, DocumentOperations, ExecutableDocument, Field, FragmentDefinition,
    FragmentSpread, InlineFragment, Name, OperationDefinition, OperationType, Positioned,
    Selection, SelectionSet, TypeCondition, Value,
};

#[test]
fn compact_query_minimal_and_unnamed() {
    //* Given
    let query = r#"{hello}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(
        ast,
        ExecutableDocument {
            operations: DocumentOperations::Single(Positioned {
                node: OperationDefinition {
                    ty: OperationType::Query,
                    selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..},
                    ..
                },
                ..
            }),
           ..
        } => {
            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                assert_eq!(name.node, Name::new("hello"));
            });
        }
    );
}

#[test]
fn compact_query_named() {
    //* Given
    let query = r#"query MyQuery{hello}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {operations: DocumentOperations::Multiple(operations),..} => {
        assert_eq!(operations.len(), 1);
        assert_matches!(operations.get(&Name::new("MyQuery")), Some(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}) => {
            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                assert_eq!(name.node, Name::new("hello"));
            });
        });

    });
}

#[test]
fn compact_query_multiple_named_queries() {
    //* Given
    let query = r#"query First{field1}query Second{field2}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Multiple(operations),
        ..
    } => {
        // Assert that the document contains two operations
        assert_eq!(operations.len(), 2);

        // Assert that the first operation is a "query" named "First" and contains a single field named "field1"
        assert_matches!(operations.get(&Name::new("First")), Some(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}) => {
            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                assert_eq!(name.node, Name::new("field1"));
            });
        });

        // Assert that the second operation is a "query" named "Second" and contains a single field named "field2"
        assert_matches!(operations.get(&Name::new("Second")), Some(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}) => {
            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                assert_eq!(name.node, Name::new("field2"));
            });
        });
    });
}

#[test]
fn compact_query_multiple_fields() {
    //* Given
    let query = r#"{field1,field2}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 2);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field1"));
        });
        assert_matches!(&selection_set[1].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field2"));
        });
    });
}

#[test]
fn compact_query_named_field_selections_nested() {
    //* Given
    let query = r#"query MyQuery{field1{field2{field3}}}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Multiple(operations),
        ..
    } => {
        assert_eq!(operations.len(), 1);
        assert_matches!(operations.get(&Name::new("MyQuery")), Some(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set1, ..}, ..}, ..}, ..}) => {
            assert_eq!(selection_set1.len(), 1);
            assert_matches!(&selection_set1[0].node, Selection::Field(Positioned { node: Field { name, selection_set: Positioned { node: SelectionSet { items: selection_set2, ..}, ..}, .. }, ..}) => {
                assert_eq!(name.node, Name::new("field1"));
                assert_eq!(selection_set2.len(), 1);
                assert_matches!(&selection_set2[0].node, Selection::Field(Positioned { node: Field { name, selection_set: Positioned { node: SelectionSet { items: selection_set3, ..}, ..}, .. }, ..}) => {
                    assert_eq!(name.node, Name::new("field2"));
                    assert_eq!(selection_set3.len(), 1);
                    assert_matches!(&selection_set3[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                        assert_eq!(name.node, Name::new("field3"));
                    });
                });
            });
        });
    });
}

#[test]
fn compact_query_named_with_field_alias() {
    //* Given
    let query = r#"query MyQuery{an_alias:field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Multiple(operations),
        ..
    } => {
        assert_eq!(operations.len(), 1);
        assert_matches!(operations.get(&Name::new("MyQuery")), Some(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}) => {
            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { alias, name, .. }, ..}) => {
                assert_matches!(alias, Some(alias) => {
                    assert_eq!(alias.node, "an_alias");
                });
                assert_eq!(name.node, Name::new("field"));
            });
        });
    });
}

#[test]
fn compact_query_field_single_argument() {
    //* Given
    let query = r#"{field(arg:42)}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, arguments, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
            assert_eq!(arguments.len(), 1);
            assert_matches!(&arguments[0], (name, value) => {
                assert_eq!(name.node, Name::new("arg"));
                assert_matches!(&value.node, Value::Number(num) => {
                    assert_matches!(num.as_u64(), Some(42));
                });
            });
        });
    });
}

#[test]
fn compact_query_field_multiple_number_arguments() {
    //* Given
    let query = r#"{field(id:-1,id1:-0,id2:0,id3:-1.23,id4:1.23,id5:1.23e+2,id6:0.123)}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, arguments, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
            assert_eq!(arguments.len(), 7);
            assert_matches!(&arguments[0], (name, value) => {
                assert_eq!(name.node, Name::new("id"));
                assert_matches!(&value.node, Value::Number(num) => {
                    assert_matches!(num.as_i64(), Some(-1)); // -1
                });
            });
            assert_matches!(&arguments[1], (name, value) => {
                assert_eq!(name.node, Name::new("id1"));
                assert_matches!(&value.node, Value::Number(num) => {
                    assert_matches!(num.as_f64(), Some(0.0)); // -0 (f64, not i64)
                });
            });
            assert_matches!(&arguments[2], (name, value) => {
                assert_eq!(name.node, Name::new("id2"));
                assert_matches!(&value.node, Value::Number(num) => {
                    assert_matches!(num.as_u64(), Some(0)); // 0
                });
            });
            assert_matches!(&arguments[3], (name, value) => {
                assert_eq!(name.node, Name::new("id3"));
                assert_matches!(&value.node, Value::Number(num) => {
                    assert_matches!(num.as_f64(), Some(-1.23)); // -1.23
                });
            });
            assert_matches!(&arguments[4], (name, value) => {
                assert_eq!(name.node, Name::new("id4"));
                assert_matches!(&value.node, Value::Number(num) => {
                    assert_matches!(num.as_f64(), Some(1.23)); // 1.23
                });
            });
            assert_matches!(&arguments[5], (name, value) => {
                assert_eq!(name.node, Name::new("id5"));
                assert_matches!(&value.node, Value::Number(num) => {
                    assert_matches!(num.as_f64(), Some(123.0)); // 1.23e+2
                });
            });
            assert_matches!(&arguments[6], (name, value) => {
                assert_eq!(name.node, Name::new("id6"));
                assert_matches!(&value.node, Value::Number(num) => {
                    assert_matches!(num.as_f64(), Some(0.123)); // 0.123
                });
            });
        });
    });
}

#[test]
fn compact_query_field_string_literal_argument() {
    //* Given
    let query = r#"{field(arg:"hello, world!")}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, arguments, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
            assert_eq!(arguments.len(), 1);
            assert_matches!(&arguments[0], (name, value) => {
                assert_eq!(name.node, Name::new("arg"));
                assert_matches!(&value.node, Value::String(value) => {
                    assert_eq!(value, "hello, world!");
                });
            });
        });
    });
}

#[test]
fn compact_query_field_multiline_string_literal_argument() {
    //* Given
    let query = indoc::indoc! {
        r#"{
            rust(arg: """
            My name
              is

            Ferris
            """)
        }"#
    };

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, arguments, .. }, ..}) => {
            assert_eq!(name.node, Name::new("rust"));
            assert_eq!(arguments.len(), 1);
            assert_matches!(&arguments[0], (name, value) => {
                assert_eq!(name.node, Name::new("arg"));
                assert_matches!(&value.node, Value::String(value) => {
                    assert_eq!(value, "My name\n  is\n\nFerris");
                });
            });
        });
    });
}

#[test]
fn compact_query_field_boolean_literal_argument() {
    //* Given
    let query = r#"{field(arg:true){field2}}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(
        ast,
        ExecutableDocument {
            operations: DocumentOperations::Single(Positioned {node: OperationDefinition {ty: OperationType::Query,selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..},..},..}),
            ..
        } => {
            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, arguments, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, .. }, ..}) => {
                assert_eq!(name.node, Name::new("field"));
                assert_eq!(arguments.len(), 1);
                assert_matches!(&arguments[0], (name, value) => {
                    assert_eq!(name.node, Name::new("arg"));
                    assert_matches!(&value.node, Value::Boolean(value) => {
                        assert_eq!(value, &true);
                    });
                });
                assert_eq!(selection_set.len(), 1);
                assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                    assert_eq!(name.node, Name::new("field2"));
                });
            });
        }
    );
}

#[test]
fn compact_query_field_with_enum_argument() {
    //* Given
    let query = r#"{field(arg:ACTIVE){field2}}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast,ExecutableDocument {
        operations: DocumentOperations::Single(Positioned {node: OperationDefinition {ty: OperationType::Query,selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..},..},..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned {
            node: Field {
                name,
                arguments,
                selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..},
                ..
            },
            ..
        }) => {
            assert_eq!(name.node, Name::new("field"));
            assert_eq!(arguments.len(), 1);
            assert_matches!(&arguments[0], (name, value) => {
                assert_eq!(name.node, Name::new("arg"));
                assert_matches!(&value.node, Value::Enum(value) => {
                    assert_eq!(value, "ACTIVE");
                });
            });
            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                assert_eq!(name.node, Name::new("field2"));
            });
        });
    });
}

#[test]
fn compact_query_field_list_argument() {
    //* Given
    let query = r#"{field(id:1,argv:[1,2,3])}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, arguments, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
            assert_eq!(arguments.len(), 2);
            assert_matches!(&arguments[0], (name, value) => {
                assert_eq!(name.node, Name::new("id"));
                assert_matches!(&value.node, Value::Number(num) => {
                    assert_matches!(num.as_u64(), Some(1));
                });
            });
            assert_matches!(&arguments[1], (name, value) => {
                assert_eq!(name.node, Name::new("argv"));
                assert_matches!(&value.node, Value::List(values) => {
                    assert_eq!(values.len(), 3);
                    assert_matches!(&values[0], Value::Number(num) => {
                        assert_matches!(num.as_u64(), Some(1));
                    });
                    assert_matches!(&values[1], Value::Number(num) => {
                        assert_matches!(num.as_u64(), Some(2));
                    });
                    assert_matches!(&values[2], Value::Number(num) => {
                        assert_matches!(num.as_u64(), Some(3));
                    });
                });
            });
        });
    });
}

#[test]
fn compact_query_field_list_argument_nested() {
    //* Given
    let query = r#"{field(id:1,argv:[[1,2],[3,4]])}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, arguments, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
            assert_eq!(arguments.len(), 2);
            assert_matches!(&arguments[0], (name, value) => {
                assert_eq!(name.node, Name::new("id"));
                assert_matches!(&value.node, Value::Number(num) => {
                    assert_matches!(num.as_u64(), Some(1));
                });
            });
            assert_matches!(&arguments[1], (name, value) => {
                assert_eq!(name.node, Name::new("argv"));
                assert_matches!(&value.node, Value::List(values) => {
                    assert_eq!(values.len(), 2);
                    assert_matches!(&values[0], Value::List(values) => {
                        assert_eq!(values.len(), 2);
                        assert_matches!(&values[0], Value::Number(num) => {
                            assert_matches!(num.as_u64(), Some(1));
                        });
                        assert_matches!(&values[1], Value::Number(num) => {
                            assert_matches!(num.as_u64(), Some(2));
                        });
                    });
                    assert_matches!(&values[1], Value::List(values) => {
                        assert_eq!(values.len(), 2);
                        assert_matches!(&values[0], Value::Number(num) => {
                            assert_matches!(num.as_u64(), Some(3));
                        });
                        assert_matches!(&values[1], Value::Number(num) => {
                            assert_matches!(num.as_u64(), Some(4));
                        });
                    });
                });
            });
        });
    });
}

#[test]
fn compact_query_field_object_argument() {
    //* Given
    let query = r#"{field(arg:{key1:"value",key2:1.61803})}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, arguments, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
            assert_eq!(arguments.len(), 1);
            assert_matches!(&arguments[0], (name, value) => {
                assert_eq!(name.node, Name::new("arg"));
                assert_matches!(&value.node, Value::Object(values) => {
                    assert_eq!(values.len(), 2);
                    let values_slice = values.iter().collect::<Vec<_>>();
                    assert_matches!(values_slice[0], (name, value) => {
                        assert_eq!(name, &Name::new("key1"));
                        assert_matches!(value, Value::String(value) => {
                            assert_eq!(value, "value");
                        });
                    });
                    assert_matches!(values_slice[1], (name, value) => {
                        assert_eq!(name, &Name::new("key2"));
                        assert_matches!(value, Value::Number(num) => {
                            assert_matches!(num.as_f64(), Some(1.61803)); // Golden ratio (φ)
                        });
                    });
                });
            });
        });
    });
}

#[test]
fn compact_query_field_object_argument_nested() {
    //* Given
    let query = r#"{field(arg:{key1:{key2:"value"}})}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, arguments, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
            assert_eq!(arguments.len(), 1);
            assert_matches!(&arguments[0], (name, value) => {
                assert_eq!(name.node, Name::new("arg"));
                assert_matches!(&value.node, Value::Object(values) => {
                    assert_eq!(values.len(), 1);
                    let values_slice = values.iter().collect::<Vec<_>>();
                    assert_matches!(values_slice[0], (name, value) => {
                        assert_eq!(name, &Name::new("key1"));
                        assert_matches!(value, Value::Object(values) => {
                            assert_eq!(values.len(), 1);
                            let values_slice = values.iter().collect::<Vec<_>>();
                            assert_matches!(values_slice[0], (name, value) => {
                                assert_eq!(name, &Name::new("key2"));
                                assert_matches!(value, Value::String(value) => {
                                    assert_eq!(value, "value");
                                });
                            });
                        });
                    });
                });
            });
        });
    });
}

#[test]
fn compact_query_field_with_directive() {
    //* Given
    let query = r#"{field@defer{count}}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, directives, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
            assert_eq!(directives.len(), 1);
            assert_matches!(&directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("defer"));
                assert_eq!(directive.arguments.len(), 0);
            });
        });
    });
}

#[test]
fn compact_query_field_with_directive_args() {
    //* Given
    let query = r#"{field@skip(if:$foo,nullable:null)}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, directives, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
            assert_eq!(directives.len(), 1);
            assert_matches!(&directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("skip"));
                assert_eq!(directive.arguments.len(), 2);
                assert_matches!(&directive.arguments[0], (name, value) => {
                    assert_eq!(name.node, Name::new("if"));
                    assert_matches!(&value.node, Value::Variable(name) => {
                        assert_eq!(name, &Name::new("foo"));
                    });
                });
                assert_matches!(&directive.arguments[1], (name, value) => {
                    assert_eq!(name.node, Name::new("nullable"));
                    assert_matches!(&value.node, Value::Null);
                });
            });
        });
    });
}

#[test]
fn compact_query_field_with_multiple_directives() {
    //* Given
    let query = r#"{field@defer@skip(if:$foo){count}}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, directives, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
            assert_eq!(directives.len(), 2);
            assert_matches!(&directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("defer"));
                assert_eq!(directive.arguments.len(), 0);
            });
            assert_matches!(&directives[1], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("skip"));
                assert_eq!(directive.arguments.len(), 1);
                assert_matches!(&directive.arguments[0], (name, value) => {
                    assert_eq!(name.node, Name::new("if"));
                    assert_matches!(&value.node, Value::Variable(name) => {
                        assert_eq!(name, &Name::new("foo"));
                    });
                });
            });
        });
    });
}

#[test]
fn compact_query_with_directive() {
    //* Given
    let query = r#"query@dir{field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(
        ast,
        ExecutableDocument {
            operations: DocumentOperations::Single(Positioned {
                node: OperationDefinition {
                    ty: OperationType::Query,
                    directives,
                    selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..},
                    ..
                },
                ..
            }),
            ..
        } => {
            assert_eq!(directives.len(), 1);
            assert_matches!(&directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("dir"));
                assert_eq!(directive.arguments.len(), 0);
            });

            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                assert_eq!(name.node, Name::new("field"));
            });
        }
    );
}

#[test]
fn compact_query_with_directive_args() {
    //* Given
    let query = r#"query@dir(if:$foo){field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(
        ast,
        ExecutableDocument {
            operations: DocumentOperations::Single(Positioned {
                node: OperationDefinition {
                    ty: OperationType::Query,
                    directives,
                    selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..},
                    ..
                },
                ..
            }),
            ..
        } => {
            assert_eq!(directives.len(), 1);
            assert_matches!(&directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("dir"));
                assert_eq!(directive.arguments.len(), 1);
                assert_matches!(&directive.arguments[0], (name, value) => {
                    assert_eq!(name.node, Name::new("if"));
                    assert_matches!(&value.node, Value::Variable(name) => {
                        assert_eq!(name, &Name::new("foo"));
                    });
                });
            });

            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                assert_eq!(name.node, Name::new("field"));
            });
        }
    );
}

#[test]
fn compact_query_with_multiple_directives() {
    //* Given
    let query = r#"query@skip(if:$is_none)@defer{field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(
        ast,
        ExecutableDocument {
            operations: DocumentOperations::Single(Positioned {
                node: OperationDefinition {
                    ty: OperationType::Query,
                    directives,
                    selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..},
                    ..
                },
                ..
            }),
            ..
        } => {
            assert_eq!(directives.len(), 2);
            assert_matches!(&directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("skip"));
                assert_eq!(directive.arguments.len(), 1);
                assert_matches!(&directive.arguments[0], (name, value) => {
                    assert_eq!(name.node, Name::new("if"));
                    assert_matches!(&value.node, Value::Variable(name) => {
                        assert_eq!(name, &Name::new("is_none"));
                    });
                });
            });
            assert_matches!(&directives[1], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("defer"));
                assert_eq!(directive.arguments.len(), 0);
            });

            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                assert_eq!(name.node, Name::new("field"));
            });
        }
    );
}

#[test]
fn compact_query_named_with_directive() {
    //* Given
    let query = r#"query MyQuery@dir{field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(
        ast,
        ExecutableDocument {
            operations: DocumentOperations::Multiple(operations),
            ..
        } => {
            assert_eq!(operations.len(), 1);
            assert_matches!(operations.get(&Name::new("MyQuery")), Some(Positioned { node: OperationDefinition { ty: OperationType::Query, directives, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}) => {
                assert_eq!(directives.len(), 1);
                assert_matches!(&directives[0], Positioned { node: directive, ..} => {
                    assert_eq!(directive.name.node, Name::new("dir"));
                    assert_eq!(directive.arguments.len(), 0);
                });

                assert_eq!(selection_set.len(), 1);
                assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                    assert_eq!(name.node, Name::new("field"));
                });
            });
        }
    );
}

#[test]
fn compact_query_with_variables() {
    //* Given
    let query = r#"query($foo:String!){field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, variable_definitions, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(variable_definitions.len(), 1);
        assert_matches!(&variable_definitions[0], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("foo"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::Named(name) => {
                    assert_eq!(name, &Name::new("String"));
                });
                assert!(!var_type.nullable);
            });
        });

        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
    });
}

#[test]
fn compact_query_with_multiple_variables() {
    //* Given
    let query = r#"query($foo:String!,$bar:Int){field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, variable_definitions, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(variable_definitions.len(), 2);
        assert_matches!(&variable_definitions[0], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("foo"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::Named(name) => {
                    assert_eq!(name, &Name::new("String"));
                });
                assert!(!var_type.nullable);
            });
        });
        assert_matches!(&variable_definitions[1], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("bar"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::Named(name) => {
                    assert_eq!(name, &Name::new("Int"));
                });
                assert!(var_type.nullable);
            });
        });

        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
    });
}

#[test]
fn compact_query_with_non_nullable_variable_type_nullable_items_list() {
    //* Given
    let query = r#"query($foo:[String]!){field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, variable_definitions, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(variable_definitions.len(), 1);
        assert_matches!(&variable_definitions[0], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("foo"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::List(list_item_type) => {
                    assert_matches!(&list_item_type.base, BaseType::Named(name) => {
                        assert_eq!(name, &Name::new("String"));
                    });
                    assert!(list_item_type.nullable);
                });
                assert!(!var_type.nullable);
            });
        });

        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
    });
}

#[test]
fn compact_query_with_nullable_variable_type_non_null_items_list() {
    //* Given
    let query = r#"query($foo:[String!]){field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, variable_definitions, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(variable_definitions.len(), 1);
        assert_matches!(&variable_definitions[0], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("foo"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::List(list_item_type) => {
                    assert_matches!(&list_item_type.base, BaseType::Named(name) => {
                        assert_eq!(name, &Name::new("String"));
                    });
                    assert!(!list_item_type.nullable);
                });
                assert!(var_type.nullable);
            });
        });

        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
    });
}

#[test]
fn compact_query_with_variable_default_value_string_literal() {
    //* Given
    let query = r#"query($foo:String!="bar"){field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, variable_definitions, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(variable_definitions.len(), 1);
        assert_matches!(&variable_definitions[0], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("foo"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::Named(name) => {
                    assert_eq!(name, &Name::new("String"));
                });
                assert!(!var_type.nullable);
            });
            assert_matches!(&variable_definition.default_value, Some(Positioned { node: value, ..}) => {
                assert_matches!(&value, ConstValue::String(const_value) => {
                    assert_eq!(const_value, "bar");
                });
            });
        });

        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
    });
}

#[test]
fn compact_query_with_variable_default_value_number_list_literal() {
    //* Given
    let query = r#"query($foo:[Int!]=[1,2,3]){field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, variable_definitions, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(variable_definitions.len(), 1);
        assert_matches!(&variable_definitions[0], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("foo"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::List(list_item_type) => {
                    assert_matches!(&list_item_type.base, BaseType::Named(name) => {
                        assert_eq!(name, &Name::new("Int"));
                    });
                    assert!(!list_item_type.nullable);
                });
                assert!(var_type.nullable);
            });
            assert_matches!(&variable_definition.default_value, Some(Positioned { node: value, ..}) => {
                assert_matches!(&value, ConstValue::List(const_values) => {
                    assert_eq!(const_values.len(), 3);
                    assert_matches!(&const_values[0], ConstValue::Number(num) => {
                        assert_matches!(num.as_u64(), Some(1));
                    });
                    assert_matches!(&const_values[1], ConstValue::Number(num) => {
                        assert_matches!(num.as_u64(), Some(2));
                    });
                    assert_matches!(&const_values[2], ConstValue::Number(num) => {
                        assert_matches!(num.as_u64(), Some(3));
                    });
                });
            });
        });

        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
    });
}

#[test]
fn compact_query_with_variable_default_value_object() {
    //* Given
    let query = r#"query($foo:KeyMap={key1:"value",key2:false}){field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, variable_definitions, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(variable_definitions.len(), 1);
        assert_matches!(&variable_definitions[0], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("foo"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::Named(name) => {
                    assert_eq!(name, &Name::new("KeyMap"));
                });
                assert!(var_type.nullable);
            });

            assert_matches!(&variable_definition.default_value, Some(Positioned { node: value, ..}) => {
                assert_matches!(&value, ConstValue::Object(fields) => {
                    assert_eq!(fields.len(), 2);
                    let fields_slice = fields.iter().collect::<Vec<_>>();
                    assert_matches!(fields_slice[0], (name, value) => {
                        assert_eq!(name, &Name::new("key1"));
                        assert_matches!(&value, ConstValue::String(value) => {
                            assert_eq!(value, "value");
                        });
                    });
                    assert_matches!(fields_slice[1], (name, value) => {
                        assert_eq!(name, &Name::new("key2"));
                        assert_matches!(&value, ConstValue::Boolean(value) => {
                            assert!(!value);
                        });
                    });
                });
            });
        });

        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
    });
}

#[test]
fn compact_query_with_variable_default_value_enum() {
    //* Given
    let query = r#"query($foo:Status!=ENABLED){field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, variable_definitions, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(variable_definitions.len(), 1);
        assert_matches!(&variable_definitions[0], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("foo"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::Named(name) => {
                    assert_eq!(name, &Name::new("Status"));
                });
                assert!(!var_type.nullable);
            });

            assert_matches!(&variable_definition.default_value, Some(Positioned { node: value, ..}) => {
                assert_matches!(&value, ConstValue::Enum(value) => {
                    assert_eq!(value, "ENABLED");
                });
            });
        });

        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
    });
}

#[test]
fn compact_query_with_variable_and_directive() {
    //* Given
    let query = r#"query($foo:String@deprecated){field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, variable_definitions, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(variable_definitions.len(), 1);
        assert_matches!(&variable_definitions[0], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("foo"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::Named(name) => {
                    assert_eq!(name, &Name::new("String"));
                });
                assert!(var_type.nullable);
            });

            assert_eq!(variable_definition.directives.len(), 1);
            assert_matches!(&variable_definition.directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("deprecated"));
                assert_eq!(directive.arguments.len(), 0);
            });
        });

        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
    });
}

#[test]
fn compact_query_with_variable_and_multiple_directives() {
    //* Given
    let query = r#"query($foo:String@lowerCase@deprecated){field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, variable_definitions, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(variable_definitions.len(), 1);
        assert_matches!(&variable_definitions[0], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("foo"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::Named(name) => {
                    assert_eq!(name, &Name::new("String"));
                });
                assert!(var_type.nullable);
            });

            assert_eq!(variable_definition.directives.len(), 2);
            assert_matches!(&variable_definition.directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("lowerCase"));
                assert_eq!(directive.arguments.len(), 0);
            });
            assert_matches!(&variable_definition.directives[1], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("deprecated"));
                assert_eq!(directive.arguments.len(), 0);
            });
        });

        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
    });
}

#[test]
fn compact_query_with_multiple_variables_and_directives() {
    //* Given
    let query = r#"query($foo:String!@lowerCase,$bar:String!@upperCase@deprecated){field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, variable_definitions, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(variable_definitions.len(), 2);
        assert_matches!(&variable_definitions[0], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("foo"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::Named(name) => {
                    assert_eq!(name, &Name::new("String"));
                });
                assert!(!var_type.nullable);
            });

            assert_eq!(variable_definition.directives.len(), 1);
            assert_matches!(&variable_definition.directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("lowerCase"));
                assert_eq!(directive.arguments.len(), 0);
            });
        });
        assert_matches!(&variable_definitions[1], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("bar"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::Named(name) => {
                    assert_eq!(name, &Name::new("String"));
                });
                assert!(!var_type.nullable);
            });

            assert_eq!(variable_definition.directives.len(), 2);
            assert_matches!(&variable_definition.directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("upperCase"));
                assert_eq!(directive.arguments.len(), 0);
            });
            assert_matches!(&variable_definition.directives[1], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("deprecated"));
                assert_eq!(directive.arguments.len(), 0);
            });
        });

        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
    });
}

#[test]
// TODO: Report bug to the async-graphql project
#[ignore = "Bug in the parser"]
fn compact_query_with_variable_default_value_and_directive() {
    //* Given
    let query = r#"query($foo:String="bar"@deprecated){field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, variable_definitions, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(variable_definitions.len(), 1);
        assert_matches!(&variable_definitions[0], Positioned { node: variable_definition, ..} => {
            assert_eq!(&variable_definition.name.node, &Name::new("foo"));
            assert_matches!(&variable_definition.var_type, Positioned { node: var_type, ..} => {
                assert_matches!(&var_type.base, BaseType::Named(name) => {
                    assert_eq!(name, &Name::new("String"));
                });
                assert!(var_type.nullable);
            });
            assert_matches!(&variable_definition.default_value, Some(Positioned { node: value, ..}) => {
                assert_matches!(&value, ConstValue::String(const_value) => {
                    assert_eq!(const_value, "bar");
                });
            });

            assert_eq!(variable_definition.directives.len(), 1);
            assert_matches!(&variable_definition.directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("dir"));
                assert_eq!(directive.arguments.len(), 0);
            });
        });

        assert_eq!(selection_set.len(), 1);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
    });
}

#[test]
fn compact_query_and_fragment() {
    //* Given
    let query = r#"query{field,...MyFragment}fragment MyFragment on MyFragmentTypeName{field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        fragments,
        ..
    } => {
        assert_eq!(selection_set.len(), 2);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
        assert_matches!(&selection_set[1].node, Selection::FragmentSpread(Positioned { node: FragmentSpread { fragment_name, .. }, ..}) => {
            assert_eq!(fragment_name.node, Name::new("MyFragment"));
        });

        assert_eq!(fragments.len(), 1);
        assert_matches!(fragments.get(&Name::new("MyFragment")), Some(Positioned { node: FragmentDefinition { type_condition, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}) => {
            assert_matches!(type_condition, Positioned { node: TypeCondition { on: Positioned { node: name, .. }, ..}, ..} => {
                assert_eq!(name, &Name::new("MyFragmentTypeName"));
            });
            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                assert_eq!(name.node, Name::new("field"));
            });
        });
    });
}

#[test]
fn compact_query_and_fragment_with_directives() {
    //* Given
    let query = r#"query{field@defer,...MyFragment@skip(if:$foo)}fragment MyFragment on MyFragmentTypeName@directive{field}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        fragments,
        ..
    } => {
        assert_eq!(selection_set.len(), 2);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, directives, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
            assert_eq!(directives.len(), 1);
            assert_matches!(&directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("defer"));
                assert_eq!(directive.arguments.len(), 0);
            });
        });
        assert_matches!(&selection_set[1].node, Selection::FragmentSpread(Positioned { node: FragmentSpread { fragment_name, directives, .. }, ..}) => {
            assert_eq!(fragment_name.node, Name::new("MyFragment"));
            assert_eq!(directives.len(), 1);
            assert_matches!(&directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("skip"));
                assert_eq!(directive.arguments.len(), 1);
                assert_matches!(&directive.arguments[0], (name, value) => {
                    assert_eq!(name.node, Name::new("if"));
                    assert_matches!(&value.node, Value::Variable(name) => {
                        assert_eq!(name, &Name::new("foo"));
                    });
                });
            });
        });

        assert_eq!(fragments.len(), 1);
        assert_matches!(fragments.get(&Name::new("MyFragment")), Some(Positioned { node: FragmentDefinition { type_condition, directives, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}) => {
            assert_matches!(type_condition, Positioned { node: TypeCondition { on: Positioned { node: name, .. }, ..}, ..} => {
                assert_eq!(name, &Name::new("MyFragmentTypeName"));
            });

            assert_eq!(directives.len(), 1);
            assert_matches!(&directives[0], Positioned { node: directive, ..} => {
                assert_eq!(directive.name.node, Name::new("directive"));
                assert_eq!(directive.arguments.len(), 0);
            });

            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, directives, .. }, ..}) => {
                assert_eq!(name.node, Name::new("field"));
                assert_eq!(directives.len(), 0);
            });
        });
    });
}

#[test]
fn compact_query_with_field_and_inline_fragment() {
    //* Given
    let query = r#"{field,...on MyFragmentTypeName{field}}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 2);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
        assert_matches!(&selection_set[1].node, Selection::InlineFragment(Positioned { node: InlineFragment { type_condition, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}) => {
            assert_matches!(type_condition, Some(Positioned { node: TypeCondition { on: Positioned { node: name, .. }, ..}, ..}) => {
                assert_eq!(name, &Name::new("MyFragmentTypeName"));
            });
            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                assert_eq!(name.node, Name::new("field"));
            });
        });
    });
}

#[test]
fn compact_query_with_field_and_inline_fragment_no_type_condition() {
    //* Given
    let query = r#"{field,...{field}}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(ast, ExecutableDocument {
        operations: DocumentOperations::Single(Positioned { node: OperationDefinition { ty: OperationType::Query, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}),
        ..
    } => {
        assert_eq!(selection_set.len(), 2);
        assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
            assert_eq!(name.node, Name::new("field"));
        });
        assert_matches!(&selection_set[1].node, Selection::InlineFragment(Positioned { node: InlineFragment { type_condition, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}) => {
            assert!(type_condition.is_none());
            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                assert_eq!(name.node, Name::new("field"));
            });
        });
    });
}

#[test]
fn compact_mutation_unnamed() {
    //* Given
    let query = r#"mutation{dropTable}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(
        ast,
        ExecutableDocument {
            operations: DocumentOperations::Single(Positioned {
                node: OperationDefinition {
                    ty: OperationType::Mutation,
                    selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..},
                    ..
                },
                ..
            }),
            ..
        } => {
            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                assert_eq!(name.node, Name::new("dropTable"));
            });
        }
    );
}

#[test]
fn compact_mutation_named() {
    //* Given
    let query = r#"mutation MyMutation{dropTable}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(
        ast,
        ExecutableDocument {
            operations: DocumentOperations::Multiple(operations),
            ..
        } => {
            assert_eq!(operations.len(), 1);
            assert_matches!(operations.get(&Name::new("MyMutation")), Some(Positioned { node: OperationDefinition { ty: OperationType::Mutation, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}) => {
                assert_eq!(selection_set.len(), 1);
                assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                    assert_eq!(name.node, Name::new("dropTable"));
                });
            });
        }
    );
}

#[test]
fn compact_subscription_unnamed() {
    //* Given
    let query = r#"subscription{newBlock}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(
        ast,
        ExecutableDocument {
            operations: DocumentOperations::Single(Positioned {
                node: OperationDefinition {
                    ty: OperationType::Subscription,
                    selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..},
                    ..
                },
                ..
            }),
            ..
        } => {
            assert_eq!(selection_set.len(), 1);
            assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                assert_eq!(name.node, Name::new("newBlock"));
            });
        }
    );
}

#[test]
fn compact_subscription_named() {
    //* Given
    let query = r#"subscription MySubscription{newBlock}"#;

    //* When
    let parsed = parse_query(query);

    //* Then
    let ast = parsed.expect("Failed to parse query");
    assert_matches!(
        ast,
        ExecutableDocument {
            operations: DocumentOperations::Multiple(operations),
            ..
        } => {
            assert_eq!(operations.len(), 1);
            assert_matches!(operations.get(&Name::new("MySubscription")), Some(Positioned { node: OperationDefinition { ty: OperationType::Subscription, selection_set: Positioned { node: SelectionSet { items: selection_set, ..}, ..}, ..}, ..}) => {
                assert_eq!(selection_set.len(), 1);
                assert_matches!(&selection_set[0].node, Selection::Field(Positioned { node: Field { name, .. }, ..}) => {
                    assert_eq!(name.node, Name::new("newBlock"));
                });
            });
        }
    );
}
