use assert_matches::assert_matches;
use graphql_toolkit_ast::{
    BaseType, ConstValue, DocumentOperations, Name, OperationType, Selection, SelectionSet, Value,
};
use graphql_toolkit_parser::parse_query as parse_exec_document;
use testlib_parser_testdata as testdata;

#[test]
fn field_with_alias() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ALIAS;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => field,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.node.name.node, Name::new("field"));
    assert_matches!(&field.node.alias, Some(alias) => {
        assert_eq!(alias.node, Name::new("field_alias"));
    });
}

#[test]
fn field_with_single_argument() {
    //* Given
    let document = testdata::exec::FIELD_WITH_SINGLE_ARGUMENT;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));
    assert_eq!(field.arguments.len(), 1);

    let (arg_name, arg_value) = &field.arguments[0];

    assert_eq!(arg_name.node, Name::new("arg"));
    assert_matches!(&arg_value.node, Value::Number(num) => {
        assert_matches!(num.as_u64(), Some(42));
    });
}

#[test]
fn field_with_arguments_null() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_NULL;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));

    assert_eq!(field.arguments.len(), 1);

    let (arg_name, arg_value) = &field.arguments[0];

    assert_eq!(arg_name.node, Name::new("arg"));
    assert_matches!(&arg_value.node, Value::Null);
}

#[test]
fn field_with_arguments_boolean() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_BOOLEAN;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));

    assert_eq!(field.arguments.len(), 1);

    let (arg_name, arg_value) = &field.arguments[0];

    assert_eq!(arg_name.node, Name::new("arg"));
    assert_matches!(&arg_value.node, Value::Boolean(value) => {
        assert_eq!(value, &true);
    });
}

#[test]
fn field_with_arguments_numbers() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_NUMBERS;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));

    assert_eq!(field.arguments.len(), 7);

    let (arg_name1, arg_value1) = &field.arguments[0];
    assert_eq!(arg_name1.node, Name::new("id"));
    assert_matches!(&arg_value1.node, Value::Number(num) => {
        assert_matches!(num.as_i64(), Some(-1)); // -1
    });

    let (arg_name2, arg_value2) = &field.arguments[1];
    assert_eq!(arg_name2.node, Name::new("id1"));
    assert_matches!(&arg_value2.node, Value::Number(num) => {
        assert_matches!(num.as_f64(), Some(0.0)); // -0 (f64, not i64)
    });

    let (arg_name3, arg_value3) = &field.arguments[2];
    assert_eq!(arg_name3.node, Name::new("id2"));
    assert_matches!(&arg_value3.node, Value::Number(num) => {
        assert_matches!(num.as_u64(), Some(0)); // 0
    });

    let (arg_name4, arg_value4) = &field.arguments[3];
    assert_eq!(arg_name4.node, Name::new("id3"));
    assert_matches!(&arg_value4.node, Value::Number(num) => {
        assert_matches!(num.as_f64(), Some(-1.23)); // -1.23
    });

    let (arg_name5, arg_value5) = &field.arguments[4];
    assert_eq!(arg_name5.node, Name::new("id4"));
    assert_matches!(&arg_value5.node, Value::Number(num) => {
        assert_matches!(num.as_f64(), Some(1.23)); // 1.23
    });

    let (arg_name6, arg_value6) = &field.arguments[5];
    assert_eq!(arg_name6.node, Name::new("id5"));
    assert_matches!(&arg_value6.node, Value::Number(num) => {
        assert_matches!(num.as_f64(), Some(123.0)); // 1.23e+2
    });

    let (arg_name7, arg_value7) = &field.arguments[6];
    assert_eq!(arg_name7.node, Name::new("id6"));
    assert_matches!(&arg_value7.node, Value::Number(num) => {
        assert_matches!(num.as_f64(), Some(0.123)); // 0.123
    });
}

#[test]
fn field_with_arguments_string_literal() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_STRING_LITERAL;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));

    let (arg_name, arg_value) = &field.arguments[0];
    assert_eq!(arg_name.node, Name::new("arg"));
    assert_matches!(&arg_value.node, Value::String(value) => {
        assert_eq!(value, "hello, world!");
    });
}

#[test]
fn field_with_arguments_string_literal_multiline() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_STRING_LITERAL_MULTILINE;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));

    let (arg_name, arg_value) = &field.arguments[0];

    assert_eq!(arg_name.node, Name::new("arg"));
    assert_matches!(&arg_value.node, Value::String(value) => {
        assert_eq!(value, "My name\n  is\n\nFerris");
    });
}

#[test]
fn field_with_arguments_enum() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_ENUM;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));

    let (arg_name, arg_value) = &field.arguments[0];
    assert_eq!(arg_name.node, Name::new("arg"));
    assert_matches!(&arg_value.node, Value::Enum(value) => {
        assert_eq!(value, "ACTIVE");
    });
}

#[test]
fn field_with_arguments_variable() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_VARIABLE;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.node.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => field,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.node.name.node, Name::new("field"));

    assert_eq!(field.node.arguments.len(), 1);

    let (arg_name, arg_value) = &field.node.arguments[0];

    assert_eq!(arg_name.node, Name::new("arg"));
    assert_matches!(&arg_value.node, Value::Variable(name) => {
        assert_eq!(name, &Name::new("var"));
    });
}

#[test]
fn field_with_arguments_list() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_LIST;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));
    assert_eq!(field.arguments.len(), 2);

    let (arg_name1, arg_value1) = &field.arguments[0];
    assert_eq!(arg_name1.node, Name::new("id"));
    assert_matches!(&arg_value1.node, Value::Number(num) => {
        assert_matches!(num.as_u64(), Some(1));
    });

    let (arg_name2, arg_value2) = &field.arguments[1];
    assert_eq!(arg_name2.node, Name::new("argv"));
    assert_matches!(&arg_value2.node, Value::List(values) => {
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
}

#[test]
fn field_with_arguments_list_single_item() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_LIST_SINGLE_ITEM;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));

    assert_eq!(field.arguments.len(), 2);

    let (arg_name1, arg_value1) = &field.arguments[0];
    assert_eq!(arg_name1.node, Name::new("id"));
    assert_matches!(&arg_value1.node, Value::Number(num) => {
        assert_matches!(num.as_u64(), Some(1));
    });

    let (arg_name2, arg_value2) = &field.arguments[1];
    assert_eq!(arg_name2.node, Name::new("argv"));
    assert_matches!(&arg_value2.node, Value::List(values) => {
        assert_eq!(values.len(), 1);
        assert_matches!(&values[0], Value::Number(num) => {
            assert_matches!(num.as_u64(), Some(1));
        });
    });
}

#[test]
fn field_with_arguments_list_nested() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_LIST_NESTED;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));

    assert_eq!(field.arguments.len(), 2);

    let (arg_name1, arg_value1) = &field.arguments[0];
    assert_eq!(arg_name1.node, Name::new("argv"));
    assert_matches!(&arg_value1.node, Value::List(values) => {
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

    let (arg_name2, arg_value2) = &field.arguments[1];
    assert_eq!(arg_name2.node, Name::new("kwargs"));
    assert_matches!(arg_value2.node, Value::Null);
}

#[test]
fn field_with_arguments_object() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_OBJECT;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));

    assert_eq!(field.arguments.len(), 2);

    let (arg_name1, arg_value1) = &field.arguments[0];
    assert_eq!(arg_name1.node, Name::new("id"));
    assert_matches!(&arg_value1.node, Value::Number(num) => {
        assert_matches!(num.as_u64(), Some(1));
    });

    let (arg_name2, arg_value2) = &field.arguments[1];
    assert_eq!(arg_name2.node, Name::new("kwargs"));

    let object = match &arg_value2.node {
        Value::Object(values) => values,
        _ => panic!("Expected an object"),
    };

    assert_eq!(object.len(), 2);
    let object_pairs = object.iter().collect::<Vec<_>>();

    let (key1, value1) = object_pairs[0];
    assert_eq!(key1, &Name::new("key1"));
    assert_matches!(value1, Value::String(value) => {
        assert_eq!(value, "value");
    });

    let (key2, value2) = object_pairs[1];
    assert_eq!(key2, &Name::new("key2"));
    assert_matches!(value2, Value::Number(num) => {
        assert_matches!(num.as_f64(), Some(1.61803)); // Golden ratio (φ)
    });
}

#[test]
fn field_with_arguments_object_single_pair() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_OBJECT_SINGLE_PAIR;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.node.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => field,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.node.name.node, Name::new("field"));

    assert_eq!(field.node.arguments.len(), 1);

    let (arg_name, arg_value) = &field.node.arguments[0];
    assert_eq!(arg_name.node, Name::new("kwargs"));

    let object = match &arg_value.node {
        Value::Object(values) => values,
        _ => panic!("Expected an object"),
    };

    assert_eq!(object.len(), 1);
    let object_pairs = object.iter().collect::<Vec<_>>();

    let (key, value) = object_pairs[0];
    assert_eq!(key, &Name::new("key"));
    assert_matches!(value, Value::String(value) => {
        assert_eq!(value, "value");
    });
}

#[test]
fn field_with_arguments_object_nested() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_OBJECT_NESTED;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.node.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => field,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.node.name.node, Name::new("field"));

    assert_eq!(field.node.arguments.len(), 2);

    let (arg_name1, arg_value1) = &field.node.arguments[0];
    assert_eq!(arg_name1.node, Name::new("kwargs"));

    let object1 = match &arg_value1.node {
        Value::Object(values) => values,
        _ => panic!("Expected an object"),
    };

    assert_eq!(object1.len(), 2);
    let object1_pairs = object1.iter().collect::<Vec<_>>();

    let (key11, value11) = object1_pairs[0];
    assert_eq!(key11, &Name::new("key11"));

    let object2 = match value11 {
        Value::Object(values) => values,
        _ => panic!("Expected an object"),
    };
    assert_eq!(object2.len(), 1);
    let object2_pairs = object2.iter().collect::<Vec<_>>();

    let (key2, value2) = object2_pairs[0];
    assert_eq!(key2, &Name::new("key2"));
    assert_matches!(value2, Value::String(value) => {
        assert_eq!(value, "value2");
    });

    let (key12, value12) = object1_pairs[1];
    assert_eq!(key12, &Name::new("key12"));
    assert_matches!(value12, Value::String(value) => {
        assert_eq!(value, "value12");
    });

    let (arg_name2, arg_value2) = &field.node.arguments[1];
    assert_eq!(arg_name2.node, Name::new("meta"));
    assert_matches!(arg_value2.node, Value::Null);
}

#[test]
fn field_with_directives() {
    //* Given
    let document = testdata::exec::FIELD_WITH_DIRECTIVES;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.node.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => field,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.node.name.node, Name::new("field"));

    assert_eq!(field.node.directives.len(), 2);

    let directive1 = &field.node.directives[0].node;
    assert_eq!(directive1.name.node, Name::new("dir1"));
    assert_eq!(directive1.arguments.len(), 0);

    let directive2 = &field.node.directives[1].node;
    assert_eq!(directive2.name.node, Name::new("dir2"));
    assert_eq!(directive2.arguments.len(), 0);
}

#[test]
fn field_with_directives_with_arguments() {
    //* Given
    let document = testdata::exec::FIELD_WITH_DIRECTIVES_WITH_ARGUMENTS;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.node.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => field,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.node.name.node, Name::new("field"));

    assert_eq!(field.node.directives.len(), 1);

    let directive = &field.node.directives[0].node;
    assert_eq!(directive.name.node, Name::new("skip"));
    assert_eq!(directive.arguments.len(), 1);

    let (dir_arg_name, dir_arg_value) = &directive.arguments[0];
    assert_eq!(dir_arg_name.node, Name::new("if"));
    assert_matches!(&dir_arg_value.node, Value::Variable(value) => {
        assert_eq!(value, &Name::new("skip"));
    });
}

#[test]
fn field_with_arguments_and_directives() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_AND_DIRECTIVES;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.node.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => field,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.node.name.node, Name::new("field"));

    assert_eq!(field.node.arguments.len(), 1);

    let (arg_name, arg_value) = &field.node.arguments[0];
    assert_eq!(arg_name.node, Name::new("id"));
    assert_matches!(&arg_value.node, Value::Number(num) => {
        assert_matches!(num.as_u64(), Some(42));
    });

    assert_eq!(field.node.directives.len(), 1);

    let directive = &field.node.directives[0].node;

    assert_eq!(directive.name.node, Name::new("skip"));
    assert_eq!(directive.arguments.len(), 1);

    let (dir_arg_name, dir_arg_value) = &directive.arguments[0];
    assert_eq!(dir_arg_name.node, Name::new("if"));
    assert_matches!(&dir_arg_value.node, Value::Variable(value) => {
        assert_eq!(value, &Name::new("skip"));
    });
}

#[test]
fn field_with_arguments_and_directives_and_selection_set() {
    //* Given
    let document = testdata::exec::FIELD_WITH_ARGUMENTS_AND_DIRECTIVES_AND_SELECTION_SET;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.node.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field1 = match &op_selection_set[0].node {
        Selection::Field(field) => field,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field1.node.name.node, Name::new("field1"));

    assert_eq!(field1.node.arguments.len(), 1);

    let (arg_name, arg_value) = &field1.node.arguments[0];
    assert_eq!(arg_name.node, Name::new("id"));
    assert_matches!(&arg_value.node, Value::Number(num) => {
        assert_matches!(num.as_u64(), Some(42));
    });

    assert_eq!(field1.node.directives.len(), 1);

    let directive = &field1.node.directives[0].node;

    assert_eq!(directive.name.node, Name::new("skip"));
    assert_eq!(directive.arguments.len(), 1);

    let (dir_arg_name, dir_arg_value) = &directive.arguments[0];
    assert_eq!(dir_arg_name.node, Name::new("if"));
    assert_matches!(&dir_arg_value.node, Value::Variable(value) => {
        assert_eq!(value, &Name::new("skip"));
    });

    let SelectionSet {
        items: selection_set1,
        ..
    } = &field1.node.selection_set.node;

    assert_eq!(selection_set1.len(), 1);

    let field2 = match &selection_set1[0].node {
        Selection::Field(field) => field,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field2.node.name.node, Name::new("field2"));
}

#[test]
fn fragment_spread() {
    //* Given
    let document = testdata::exec::FRAGMENT_SPREAD;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.node.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let fragment_spread = match &op_selection_set[0].node {
        Selection::FragmentSpread(fragment_spread) => &fragment_spread.node,
        _ => panic!("Expected a fragment spread"),
    };
    assert_eq!(fragment_spread.fragment_name.node, Name::new("MyFragment"));
    assert_eq!(fragment_spread.directives.len(), 0);
}

#[test]
fn fragment_spread_with_directives() {
    //* Given
    let document = testdata::exec::FRAGMENT_SPREAD_WITH_DIRECTIVES;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.node.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let fragment_spread = match &op_selection_set[0].node {
        Selection::FragmentSpread(fragment_spread) => &fragment_spread.node,
        _ => panic!("Expected a fragment spread"),
    };
    assert_eq!(fragment_spread.fragment_name.node, Name::new("MyFragment"));

    assert_eq!(fragment_spread.directives.len(), 1);

    let directive = &fragment_spread.directives[0].node;
    assert_eq!(directive.name.node, Name::new("directive"));
    assert_eq!(directive.arguments.len(), 0);
}

#[test]
fn inline_fragment() {
    //* Given
    let document = testdata::exec::INLINE_FRAGMENT;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.node.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let inline_fragment = match &op_selection_set[0].node {
        Selection::InlineFragment(inline_fragment) => &inline_fragment.node,
        _ => panic!("Expected an inline fragment"),
    };
    assert!(inline_fragment.type_condition.is_none());
    assert_eq!(inline_fragment.directives.len(), 0);

    let SelectionSet {
        items: inline_fragment_selection_set,
        ..
    } = &inline_fragment.selection_set.node;

    assert_eq!(inline_fragment_selection_set.len(), 1);

    let field = match &inline_fragment_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn inline_fragment_with_type_condition() {
    //* Given
    let document = testdata::exec::INLINE_FRAGMENT_WITH_TYPE_CONDITION;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.node.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let inline_fragment = match &op_selection_set[0].node {
        Selection::InlineFragment(inline_fragment) => &inline_fragment.node,
        _ => panic!("Expected an inline fragment"),
    };

    let inline_fragment_type_condition = match &inline_fragment.type_condition {
        Some(type_condition) => &type_condition.node,
        None => panic!("Expected a type condition"),
    };
    assert_eq!(inline_fragment_type_condition.on.node, Name::new("MyType"));
    assert_eq!(inline_fragment.directives.len(), 0);

    let SelectionSet {
        items: inline_fragment_selection_set,
        ..
    } = &inline_fragment.selection_set.node;

    assert_eq!(inline_fragment_selection_set.len(), 1);

    let field = match &inline_fragment_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn inline_fragment_with_directives() {
    //* Given
    let document = testdata::exec::INLINE_FRAGMENT_WITH_DIRECTIVES;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.node.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let inline_fragment = match &op_selection_set[0].node {
        Selection::InlineFragment(inline_fragment) => &inline_fragment.node,
        _ => panic!("Expected an inline fragment"),
    };
    assert!(inline_fragment.type_condition.is_none());
    assert_eq!(inline_fragment.directives.len(), 1);

    let directive = &inline_fragment.directives[0].node;
    assert_eq!(directive.name.node, Name::new("directive"));
    assert_eq!(directive.arguments.len(), 0);

    let SelectionSet {
        items: inline_fragment_selection_set,
        ..
    } = &inline_fragment.selection_set.node;

    assert_eq!(inline_fragment_selection_set.len(), 1);

    let field = match &inline_fragment_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn inline_fragment_with_type_condition_and_directives() {
    //* Given
    let document = testdata::exec::INLINE_FRAGMENT_WITH_TYPE_CONDITION_AND_DIRECTIVES;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.node.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let inline_fragment = match &op_selection_set[0].node {
        Selection::InlineFragment(inline_fragment) => &inline_fragment.node,
        _ => panic!("Expected an inline fragment"),
    };

    let inline_fragment_type_condition = match &inline_fragment.type_condition {
        Some(type_condition) => &type_condition.node,
        None => panic!("Expected a type condition"),
    };
    assert_eq!(inline_fragment_type_condition.on.node, Name::new("MyType"));

    assert_eq!(inline_fragment.directives.len(), 1);

    let directive = &inline_fragment.directives[0].node;
    assert_eq!(directive.name.node, Name::new("directive"));
    assert_eq!(directive.arguments.len(), 0);

    let SelectionSet {
        items: inline_fragment_selection_set,
        ..
    } = &inline_fragment.selection_set.node;

    assert_eq!(inline_fragment_selection_set.len(), 1);

    let field = match &inline_fragment_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn selection_set_with_multiple_selections() {
    //* Given
    let document = testdata::exec::SELECTION_SET_WITH_MULTIPLE_SELECTIONS;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 3);

    let fragment_spread = match &op_selection_set[0].node {
        Selection::FragmentSpread(fragment_spread) => &fragment_spread.node,
        _ => panic!("Expected a fragment spread selection"),
    };
    assert_eq!(fragment_spread.fragment_name.node, Name::new("MyFragment"));
    assert_eq!(fragment_spread.directives.len(), 0);

    let inline_fragment = match &op_selection_set[1].node {
        Selection::InlineFragment(inline_fragment) => &inline_fragment.node,
        _ => panic!("Expected an inline fragment selection"),
    };

    let inline_fragment_type_condition = match &inline_fragment.type_condition {
        Some(type_condition) => &type_condition.node,
        None => panic!("Expected a type condition"),
    };

    assert_eq!(inline_fragment_type_condition.on.node, Name::new("MyType"));
    assert_eq!(inline_fragment.directives.len(), 0);

    let field = match &op_selection_set[2].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
#[ignore = "Allowed by the spec. Not passing for async-graphql-parser"]
fn selection_set_empty() {
    //* Given
    let document = testdata::exec::SELECTION_SET_EMPTY;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 0);
}

#[test]
fn selection_set_nesting() {
    //* Given
    let document = testdata::exec::SELECTION_SET_NESTING;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field1 = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field1.name.node, Name::new("field1"));

    let SelectionSet {
        items: selection_set1,
        ..
    } = &field1.selection_set.node;

    assert_eq!(selection_set1.len(), 2);

    let field21 = match &selection_set1[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field21.name.node, Name::new("field21"));

    let field22 = match &selection_set1[1].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field22.name.node, Name::new("field22"));

    let SelectionSet {
        items: selection_set2,
        ..
    } = &field22.selection_set.node;

    assert_eq!(selection_set2.len(), 1);

    let field3 = match &selection_set2[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field3.name.node, Name::new("field3"));
}

#[test]
fn op_query_shorthand() {
    //* Given
    let document = testdata::exec::OP_QUERY_SHORTHAND;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };
    assert_matches!(operation.ty, OperationType::Query);

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_name() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_NAME;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Multiple(operations) => operations,
        _ => panic!("Expected multiple operations"),
    };

    assert_eq!(operations.len(), 1);

    let operation = match operations.get(&Name::new("MyQuery")) {
        Some(operation) => &operation.node,
        _ => panic!("Expected an operation named 'MyQuery'"),
    };
    assert_matches!(operation.ty, OperationType::Query);

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_directives() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_DIRECTIVES;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operation.directives.len(), 3);

    let op_directive1 = &operation.directives[0].node;
    assert_eq!(op_directive1.name.node, Name::new("dir1"));
    assert_eq!(op_directive1.arguments.len(), 0);

    let op_directive2 = &operation.directives[1].node;
    assert_eq!(op_directive2.name.node, Name::new("dir2"));
    assert_eq!(op_directive2.arguments.len(), 0);

    let op_directive3 = &operation.directives[2].node;
    assert_eq!(op_directive3.name.node, Name::new("dir3"));
    assert_eq!(op_directive3.arguments.len(), 0);

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_name_and_directives() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_NAME_AND_DIRECTIVES;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Multiple(operations) => operations,
        _ => panic!("Expected multiple operations"),
    };

    assert_eq!(operations.len(), 1);

    let operation = match operations.get(&Name::new("MyQuery")) {
        Some(operation) => &operation.node,
        _ => panic!("Expected an operation named 'MyQuery'"),
    };

    assert_eq!(operation.directives.len(), 1);

    let op_directive = &operation.directives[0].node;
    assert_eq!(op_directive.name.node, Name::new("directive"));

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_variables() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 2);

    let var1 = &operations.variable_definitions[0].node;
    assert_eq!(var1.name.node, Name::new("var1"));
    assert_eq!(var1.var_type.node.base, BaseType::Named(Name::new("Int")));
    assert!(var1.var_type.node.nullable);
    assert!(var1.default_value.is_none());

    let var2 = &operations.variable_definitions[1].node;
    assert_eq!(var2.name.node, Name::new("var2"));
    assert_eq!(
        var2.var_type.node.base,
        BaseType::Named(Name::new("String"))
    );
    assert!(var2.var_type.node.nullable);
    assert!(var2.default_value.is_none());

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_variables_with_directives() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES_WITH_DIRECTIVES;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 1);

    let var = &operations.variable_definitions[0].node;
    assert_eq!(var.name.node, Name::new("var"));
    assert_eq!(var.var_type.node.base, BaseType::Named(Name::new("Int")));
    assert!(var.var_type.node.nullable);
    assert!(var.default_value.is_none());

    assert_eq!(var.directives.len(), 1);

    let var_directive = &var.directives[0].node;
    assert_eq!(var_directive.name.node, Name::new("directive"));
    assert_eq!(var_directive.arguments.len(), 0);

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_variables_single_item() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES_SINGLE_ITEM;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 1);

    let var1 = &operations.variable_definitions[0].node;
    assert_eq!(var1.name.node, Name::new("var"));
    assert_eq!(var1.var_type.node.base, BaseType::Named(Name::new("Int")));
    assert!(var1.var_type.node.nullable);
    assert!(var1.default_value.is_none());

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_variables_of_type_not_nullable() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES_OF_TYPE_NOT_NULLABLE;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 2);

    let var1 = &operations.variable_definitions[0].node;
    assert_eq!(var1.name.node, Name::new("var1"));
    assert_eq!(var1.var_type.node.base, BaseType::Named(Name::new("Int")));
    assert!(var1.var_type.node.nullable);
    assert!(var1.default_value.is_none());

    let var2 = &operations.variable_definitions[1].node;
    assert_eq!(var2.name.node, Name::new("var2"));
    assert_eq!(
        var2.var_type.node.base,
        BaseType::Named(Name::new("String"))
    );
    assert!(!var2.var_type.node.nullable);
    assert!(var2.default_value.is_none());

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_variables_of_type_list() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES_OF_TYPE_LIST;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 1);

    let var = &operations.variable_definitions[0].node;

    assert_eq!(var.name.node, Name::new("var"));

    let var_type = match &var.var_type.node.base {
        BaseType::List(list) => list,
        _ => panic!("Expected a list base type"),
    };
    assert_eq!(var_type.base, BaseType::Named(Name::new("String")));
    assert!(var_type.nullable); // Nullable

    assert!(var.var_type.node.nullable); // Nullable
    assert!(var.default_value.is_none());
}

#[test]
fn op_query_with_variables_of_type_not_nullable_list() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES_OF_TYPE_NOT_NULLABLE_LIST;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 1);

    let var = &operations.variable_definitions[0].node;

    assert_eq!(var.name.node, Name::new("var"));

    let item_type = match &var.var_type.node.base {
        BaseType::List(list) => list,
        _ => panic!("Expected a list base type"),
    };
    assert_eq!(item_type.base, BaseType::Named(Name::new("String")));
    assert!(item_type.nullable); // Nullable

    assert!(!var.var_type.node.nullable); // Not nullable
    assert!(var.default_value.is_none());
}

#[test]
fn op_query_with_variables_of_type_list_with_item_not_nullable() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES_OF_TYPE_LIST_WITH_ITEM_NOT_NULLABLE;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 1);

    let var = &operations.variable_definitions[0].node;

    assert_eq!(var.name.node, Name::new("var"));
    let item_type = match &var.var_type.node.base {
        BaseType::List(list) => list,
        _ => panic!("Expected a list base type"),
    };

    assert_eq!(item_type.base, BaseType::Named(Name::new("String")));
    assert!(!item_type.nullable); // Not nullable

    assert!(var.var_type.node.nullable); // Nullable
    assert!(var.default_value.is_none());
}

#[test]
fn op_query_with_variables_of_type_not_nullable_list_with_item_not_nullable() {
    //* Given
    let document =
        testdata::exec::OP_QUERY_WITH_VARIABLES_OF_TYPE_NOT_NULLABLE_LIST_WITH_ITEM_NOT_NULLABLE;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 1);

    let var = &operations.variable_definitions[0].node;

    assert_eq!(var.name.node, Name::new("var"));
    let item_type = match &var.var_type.node.base {
        BaseType::List(list) => list,
        _ => panic!("Expected a list base type"),
    };

    assert_eq!(item_type.base, BaseType::Named(Name::new("String")));
    assert!(!item_type.nullable); // Not nullable

    assert!(!var.var_type.node.nullable); // Not nullable
    assert!(var.default_value.is_none());
}

#[test]
fn op_query_with_variables_with_default_value() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 2);

    let var1 = &operations.variable_definitions[0].node;
    assert_eq!(var1.name.node, Name::new("var1"));
    assert_eq!(var1.var_type.node.base, BaseType::Named(Name::new("Int")));
    assert!(var1.var_type.node.nullable); // Nullable

    let var1_default_value = match &var1.default_value {
        Some(default_value) => &default_value.node,
        _ => panic!("Expected a default value"),
    };
    assert_matches!(var1_default_value, ConstValue::Number(num) => {
        assert_matches!(num.as_u64(), Some(1));
    });

    let var2 = &operations.variable_definitions[1].node;
    assert_eq!(var2.name.node, Name::new("var2"));
    assert_eq!(var2.var_type.node.base, BaseType::Named(Name::new("Int")));
    assert!(var2.var_type.node.nullable); // Nullable

    let var2_default_value = match &var2.default_value {
        Some(default_value) => &default_value.node,
        _ => panic!("Expected a default value"),
    };
    assert_matches!(var2_default_value, ConstValue::Number(num) => {
        assert_matches!(num.as_u64(), Some(2));
    });

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_variables_with_default_value_null() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE_NULL;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 1);

    let var = &operations.variable_definitions[0].node;
    assert_eq!(var.name.node, Name::new("var"));
    assert_eq!(var.var_type.node.base, BaseType::Named(Name::new("String")));
    assert!(var.var_type.node.nullable); // Nullable

    let var_default_value = match &var.default_value {
        Some(default_value) => &default_value.node,
        _ => panic!("Expected a default value"),
    };
    assert_matches!(var_default_value, ConstValue::Null);

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_variables_with_default_value_boolean() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE_BOOLEAN;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 1);

    let var = &operations.variable_definitions[0].node;
    assert_eq!(var.name.node, Name::new("var"));
    assert_eq!(
        var.var_type.node.base,
        BaseType::Named(Name::new("Boolean"))
    );
    assert!(var.var_type.node.nullable); // Nullable

    let var1_default_value = match &var.default_value {
        Some(default_value) => &default_value.node,
        _ => panic!("Expected a default value"),
    };
    assert_matches!(var1_default_value, ConstValue::Boolean(value) => {
        assert_eq!(value, &true);
    });

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_variables_with_default_value_enum() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE_ENUM;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 1);

    let var = &operations.variable_definitions[0].node;
    assert_eq!(var.name.node, Name::new("var"));
    assert_eq!(var.var_type.node.base, BaseType::Named(Name::new("Status")));
    assert!(var.var_type.node.nullable); // Nullable

    let var2_default_value = match &var.default_value {
        Some(default_value) => &default_value.node,
        _ => panic!("Expected a default value"),
    };
    assert_matches!(var2_default_value, ConstValue::Enum(value) => {
        assert_eq!(value, "ENABLED");
    });

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_variables_with_default_value_string() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE_STRING;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 2);

    let var1 = &operations.variable_definitions[0].node;
    assert_eq!(var1.name.node, Name::new("var1"));
    assert_eq!(var1.var_type.node.base, BaseType::Named(Name::new("Int")));
    assert!(var1.var_type.node.nullable); // Nullable
    assert!(var1.default_value.is_none());

    let var2 = &operations.variable_definitions[1].node;
    assert_eq!(var2.name.node, Name::new("var2"));
    assert_eq!(
        var2.var_type.node.base,
        BaseType::Named(Name::new("String"))
    );
    assert!(var2.var_type.node.nullable); // Nullable

    let var2_default_value = match &var2.default_value {
        Some(default_value) => &default_value.node,
        _ => panic!("Expected a default value"),
    };
    assert_matches!(var2_default_value, ConstValue::String(value) => {
        assert_eq!(value, "value");
    });

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_variables_with_default_value_list() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE_LIST;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 3);

    let var1 = &operations.variable_definitions[0].node;
    assert_eq!(var1.name.node, Name::new("var1"));
    let var1_item_type = match &var1.var_type.node.base {
        BaseType::List(list) => list,
        _ => panic!("Expected a list base type"),
    };
    assert_eq!(var1_item_type.base, BaseType::Named(Name::new("Int")));
    assert!(var1_item_type.nullable); // Nullable

    assert!(var1.var_type.node.nullable); // Nullable

    let var1_default_value = match &var1.default_value {
        Some(default_value) => &default_value.node,
        _ => panic!("Expected a default value"),
    };

    let var1_default_value_list = match var1_default_value {
        ConstValue::List(list) => list,
        _ => panic!("Expected a list default value"),
    };

    assert_eq!(var1_default_value_list.len(), 3);
    assert_matches!(&var1_default_value_list[0], ConstValue::Number(num) => {
        assert_matches!(num.as_u64(), Some(1));
    });
    assert_matches!(&var1_default_value_list[1], ConstValue::Number(num) => {
        assert_matches!(num.as_u64(), Some(2));
    });
    assert_matches!(&var1_default_value_list[2], ConstValue::Number(num) => {
        assert_matches!(num.as_u64(), Some(3));
    });

    let var2 = &operations.variable_definitions[1].node;
    assert_eq!(var2.name.node, Name::new("var2"));
    assert_eq!(var2.var_type.node.base, BaseType::Named(Name::new("Int")));
    assert!(var2.var_type.node.nullable); // Nullable
    assert!(var2.default_value.is_none());

    let var3 = &operations.variable_definitions[2].node;
    assert_eq!(var3.name.node, Name::new("var3"));
    assert_eq!(var3.var_type.node.base, BaseType::Named(Name::new("Float")));
    assert!(var3.var_type.node.nullable); // Nullable
    assert!(var3.default_value.is_none());

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_variables_with_default_value_object() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_VARIABLES_WITH_DEFAULT_VALUE_OBJECT;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };

    assert_eq!(operations.variable_definitions.len(), 1);

    let var1 = &operations.variable_definitions[0].node;
    assert_eq!(var1.name.node, Name::new("var1"));
    assert_eq!(
        var1.var_type.node.base,
        BaseType::Named(Name::new("KeyMap"))
    );
    assert!(var1.var_type.node.nullable); // Nullable

    let var1_default_value = match &var1.default_value {
        Some(default_value) => &default_value.node,
        _ => panic!("Expected a default value"),
    };

    let var1_default_value_object = match var1_default_value {
        ConstValue::Object(value) => value,
        _ => panic!("Expected a list default value"),
    };

    assert_eq!(var1_default_value_object.len(), 2);
    let pairs = var1_default_value_object.iter().collect::<Vec<_>>();

    let (key1, value1) = pairs[0];
    assert_eq!(*key1, Name::new("key1"));
    assert_matches!(value1, ConstValue::String(value) => {
        assert_eq!(value, "value1");
    });

    let (key2, value2) = pairs[1];
    assert_eq!(*key2, Name::new("key2"));
    assert_matches!(value2, ConstValue::String(value) => {
        assert_eq!(value, "value2");
    });

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_query_with_name_and_variables_and_directives() {
    //* Given
    let document = testdata::exec::OP_QUERY_WITH_NAME_AND_VARIABLES_AND_DIRECTIVES;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Multiple(operations) => operations,
        _ => panic!("Expected multiple operations"),
    };

    assert_eq!(operations.len(), 1);

    let operation = match operations.get(&Name::new("MyQuery")) {
        Some(operation) => &operation.node,
        _ => panic!("Expected an operation named 'MyQuery'"),
    };

    assert_eq!(operation.variable_definitions.len(), 1);

    let var = &operation.variable_definitions[0].node;
    assert_eq!(var.name.node, Name::new("var"));
    assert_eq!(var.var_type.node.base, BaseType::Named(Name::new("String")));
    assert!(var.var_type.node.nullable);

    let var_default_value = match &var.default_value {
        Some(default_value) => &default_value.node,
        _ => panic!("Expected a default value"),
    };
    assert_matches!(var_default_value, ConstValue::Null);

    assert_eq!(operation.directives.len(), 1);

    let op_directive = &operation.directives[0].node;
    assert_eq!(op_directive.name.node, Name::new("directive"));

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_mutation() {
    //* Given
    let document = testdata::exec::OP_MUTATION;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };
    assert_matches!(operation.ty, OperationType::Mutation);

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_mutation_with_name() {
    //* Given
    let document = testdata::exec::OP_MUTATION_WITH_NAME;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Multiple(operations) => operations,
        _ => panic!("Expected multiple operations"),
    };

    assert_eq!(operations.len(), 1);

    let operation = match operations.get(&Name::new("MyMutation")) {
        Some(operation) => &operation.node,
        _ => panic!("Expected an operation named 'MyMutation'"),
    };
    assert_matches!(operation.ty, OperationType::Mutation);

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_subscription() {
    //* Given
    let document = testdata::exec::OP_SUBSCRIPTION;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operation = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };
    assert_matches!(operation.ty, OperationType::Subscription);

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn op_subscription_with_name() {
    //* Given
    let document = testdata::exec::OP_SUBSCRIPTION_WITH_NAME;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Multiple(operations) => operations,
        _ => panic!("Expected multiple operations"),
    };

    assert_eq!(operations.len(), 1);

    let operation = match operations.get(&Name::new("MySubscription")) {
        Some(operation) => &operation.node,
        _ => panic!("Expected an operation named 'MySubscription'"),
    };
    assert_matches!(operation.ty, OperationType::Subscription);

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operation.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn document_multiple_op_query_with_name() {
    //* Given
    let document = testdata::exec::DOCUMENT_MULTIPLE_OP_QUERY_WITH_NAME;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Multiple(operations) => operations,
        _ => panic!("Expected multiple operations"),
    };

    // Assert that the document contains two operations
    assert_eq!(operations.len(), 2);

    // Assert that the first operation is a "query" named "First" and contains a single field named "field1"
    let op_first = match operations.get(&Name::new("First")) {
        Some(op) => &op.node,
        _ => panic!("Expected an operation named 'First'"),
    };

    let SelectionSet {
        items: selection_set,
        ..
    } = &op_first.selection_set.node;

    assert_eq!(selection_set.len(), 1);

    let field = match &selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field1"));

    // Assert that the second operation is a "query" named "Second" and contains a single field named "field2"
    let op_second = match operations.get(&Name::new("Second")) {
        Some(op) => &op.node,
        _ => panic!("Expected an operation named 'Second'"),
    };

    let SelectionSet {
        items: selection_set,
        ..
    } = &op_second.selection_set.node;

    assert_eq!(selection_set.len(), 1);

    let field = match &selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field2"));
}

#[test]
fn document_with_op_query_and_fragments() {
    //* Given
    let document = testdata::exec::DOCUMENT_WITH_OP_QUERY_AND_FRAGMENTS;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };
    assert_matches!(operations.ty, OperationType::Query);

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));

    let fragment = match ast.fragments.get(&Name::new("MyFragment")) {
        Some(fragment) => &fragment.node,
        _ => panic!("Expected a fragment named 'MyFragment'"),
    };

    assert_eq!(fragment.type_condition.node.on.node, Name::new("MyType"));

    let SelectionSet {
        items: fragment_selection_set,
        ..
    } = &fragment.selection_set.node;

    assert_eq!(fragment_selection_set.len(), 1);

    let field = match &fragment_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}

#[test]
fn document_with_op_query_and_fragments_with_directives() {
    //* Given
    let document = testdata::exec::DOCUMENT_WITH_OP_QUERY_AND_FRAGMENTS_WITH_DIRECTIVES;

    //* When
    let parsed = parse_exec_document(document);

    //* Then
    let ast = parsed.expect("Failed to parse document");
    let operations = match ast.operations {
        DocumentOperations::Single(operation) => operation.node,
        _ => panic!("Expected a single operation"),
    };
    assert_matches!(operations.ty, OperationType::Query);

    let SelectionSet {
        items: op_selection_set,
        ..
    } = &operations.selection_set.node;

    assert_eq!(op_selection_set.len(), 1);

    let field = match &op_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };

    assert_eq!(field.name.node, Name::new("field"));

    let fragment = match ast.fragments.get(&Name::new("MyFragment")) {
        Some(fragment) => &fragment.node,
        _ => panic!("Expected a fragment named 'MyFragment'"),
    };

    assert_eq!(fragment.type_condition.node.on.node, Name::new("MyType"));

    assert_eq!(fragment.directives.len(), 1);

    let fragment_directive = &fragment.directives[0].node;
    assert_eq!(fragment_directive.name.node, Name::new("directive"));
    assert_eq!(fragment_directive.arguments.len(), 0);

    let SelectionSet {
        items: fragment_selection_set,
        ..
    } = &fragment.selection_set.node;

    assert_eq!(fragment_selection_set.len(), 1);

    let field = match &fragment_selection_set[0].node {
        Selection::Field(field) => &field.node,
        _ => panic!("Expected a field selection"),
    };
    assert_eq!(field.name.node, Name::new("field"));
}
