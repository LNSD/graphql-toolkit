use std::io;

use graphql_toolkit_ast::{
    indexmap::IndexMap, BaseType, ConstValue, Directive, DocumentOperations, ExecutableDocument,
    Field, FragmentDefinition, FragmentSpread, InlineFragment, Name, Number, OperationDefinition,
    OperationType, Positioned, Selection, SelectionSet, Type, TypeCondition, Value,
    VariableDefinition,
};

use crate::fmt::formatter::Formatter;

/// A trait for serializing a GraphQL AST into a GraphQL document.
pub trait Serialize {
    /// Serialize this value into the given serializer.
    fn serialize<W, F>(&self, ser: &mut Serializer<W, F>) -> anyhow::Result<()>
    where
        W: io::Write,
        F: Formatter;
}

/// A structure for serializing Rust GraphQL AST types to GraphQL documents.
pub struct Serializer<W, F> {
    writer: W,
    formatter: F,
}

impl<W, F> Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    /// Create a new serializer with a custom formatter.
    #[inline]
    pub fn with_formatter(writer: W, formatter: F) -> Self {
        Self { writer, formatter }
    }
}

impl<W, F> Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    fn serialize_executable_document(&mut self, value: &ExecutableDocument) -> anyhow::Result<()> {
        match &value.operations {
            DocumentOperations::Single(def) => {
                self.serialize_operation_definition(&def.node, None, true)?;
            }
            DocumentOperations::Multiple(def) => {
                let mut first_operation = true;
                for (name, def) in def.iter() {
                    if !first_operation {
                        self.formatter
                            .before_operation_or_fragment_definition(&mut self.writer)?;
                    }

                    self.serialize_operation_definition(&def.node, Some(name), false)?;
                    first_operation = false;
                }
            }
        }

        for (name, fragment) in value.fragments.iter() {
            self.formatter
                .before_operation_or_fragment_definition(&mut self.writer)?;
            self.serialize_fragment_definition(name, &fragment.node)?;
        }

        Ok(())
    }

    fn serialize_operation_definition(
        &mut self,
        value: &OperationDefinition,
        name: Option<&Name>,
        single: bool,
    ) -> anyhow::Result<()> {
        // Use the "query shorthand" if a document contains *only one operation* and
        // that operation is a query which defines *no variables* and contains *no
        // directives* then that operation may be represented in a shorthand form
        // which omits the query keyword and operation name. For example:
        //
        //  { field }
        //
        // https://spec.graphql.org/October2021/#sec-Language.Operations.Query-shorthand
        let shorthand = single
            && value.ty == OperationType::Query
            && name.is_none()
            && value.variable_definitions.is_empty()
            && value.directives.is_empty();

        // Operation signature
        if !shorthand {
            // Type
            match value.ty {
                OperationType::Query => {
                    self.formatter.write_keyword(&mut self.writer, "query")?;
                }
                OperationType::Mutation => {
                    self.formatter.write_keyword(&mut self.writer, "mutation")?;
                }
                OperationType::Subscription => {
                    self.formatter
                        .write_keyword(&mut self.writer, "subscription")?;
                }
            }

            // Name
            if let Some(name) = name {
                self.formatter.write_separator(&mut self.writer)?;
                name.serialize(self)?;
            }

            // Variables definition
            if name.is_none() && !value.variable_definitions.is_empty() {
                self.formatter
                    .before_operation_variable_definitions(&mut self.writer)?;
            }
            if !value.variable_definitions.is_empty() {
                self.formatter.begin_parentheses(&mut self.writer)?;

                let mut iter = value.variable_definitions.iter().peekable();
                while let Some(def) = iter.next() {
                    def.serialize(self)?;

                    // If there are more variable definitions, add a separator
                    if iter.peek().is_some() {
                        self.formatter.write_item_separator(&mut self.writer)?;
                    }
                }

                self.formatter.end_parentheses(&mut self.writer)?;
            }

            // Directives
            for directive in value.directives.iter() {
                directive.serialize(self)?;
            }

            self.formatter
                .after_operation_or_fragment_signature(&mut self.writer)?;
        }

        // Selection set
        value.selection_set.serialize(self)?;

        Ok(())
    }

    fn serialize_selection_set(&mut self, value: &SelectionSet) -> anyhow::Result<()> {
        // Empty selection sets are not serialized
        if value.items.is_empty() {
            return Ok(());
        }

        self.formatter.begin_block(&mut self.writer)?;

        let mut iter = value.items.iter().peekable();
        while let Some(selection) = iter.next() {
            self.formatter.before_block_item(&mut self.writer)?;
            selection.serialize(self)?;

            // If there are more selections, add a separator
            if iter.peek().is_some() {
                self.formatter.after_block_item(&mut self.writer)?;
            }
        }

        self.formatter.end_block(&mut self.writer)?;

        Ok(())
    }

    fn serialize_selection(&mut self, value: &Selection) -> anyhow::Result<()> {
        match value {
            Selection::Field(field) => field.serialize(self),
            Selection::FragmentSpread(fragment) => fragment.serialize(self),
            Selection::InlineFragment(fragment) => fragment.serialize(self),
        }?;

        Ok(())
    }

    fn serialize_selection_field(&mut self, value: &Field) -> anyhow::Result<()> {
        if let Some(alias) = &value.alias {
            alias.serialize(self)?;
            self.formatter
                .write_name_value_separator(&mut self.writer)?;
        }

        value.name.serialize(self)?;

        if !value.arguments.is_empty() {
            self.serialize_arguments(&value.arguments)?;
        }

        for directive in value.directives.iter() {
            directive.serialize(self)?;
        }

        if !value.selection_set.node.items.is_empty() {
            self.formatter.after_selection_signature(&mut self.writer)?;
        }
        value.selection_set.serialize(self)?;

        Ok(())
    }

    fn serialize_fragment_spread(&mut self, value: &FragmentSpread) -> anyhow::Result<()> {
        self.formatter.write_keyword(&mut self.writer, "...")?;
        value.fragment_name.serialize(self)?;

        for directive in value.directives.iter() {
            directive.serialize(self)?;
        }

        Ok(())
    }

    fn serialize_inline_fragment(&mut self, value: &InlineFragment) -> anyhow::Result<()> {
        self.formatter.write_keyword(&mut self.writer, "...")?;

        if let Some(type_condition) = &value.type_condition {
            self.formatter.before_type_condition(&mut self.writer)?;
            type_condition.serialize(self)?;
        }

        for directive in value.directives.iter() {
            directive.serialize(self)?;
        }

        if !value.selection_set.node.items.is_empty() {
            self.formatter.after_selection_signature(&mut self.writer)?;
        }
        value.selection_set.serialize(self)?;

        Ok(())
    }

    fn serialize_arguments(
        &mut self,
        value: &[(Positioned<Name>, Positioned<Value>)],
    ) -> anyhow::Result<()> {
        self.formatter.begin_parentheses(&mut self.writer)?;

        let mut iter = value.iter().peekable();
        while let Some((name, value)) = iter.next() {
            name.serialize(self)?;
            self.formatter
                .write_name_value_separator(&mut self.writer)?;
            value.serialize(self)?;

            // If there are more arguments, add a separator
            if iter.peek().is_some() {
                self.formatter.write_item_separator(&mut self.writer)?;
            }
        }

        self.formatter.end_parentheses(&mut self.writer)?;

        Ok(())
    }

    fn serialize_directive(&mut self, value: &Directive) -> anyhow::Result<()> {
        self.formatter.before_directive(&mut self.writer)?;

        // Directive name
        self.formatter.begin_directive(&mut self.writer)?;
        value.name.serialize(self)?;

        // Arguments
        if !value.arguments.is_empty() {
            self.serialize_arguments(&value.arguments)?;
        }

        Ok(())
    }

    fn serialize_variable_definition(&mut self, value: &VariableDefinition) -> anyhow::Result<()> {
        // Variable name
        self.formatter.begin_variable(&mut self.writer)?;
        value.name.serialize(self)?;

        // Variable name-type separator
        self.formatter
            .write_name_value_separator(&mut self.writer)?;

        // Variable type
        value.var_type.serialize(self)?;

        if let Some(default_value) = &value.default_value {
            self.formatter
                .write_variable_default_value_separator(&mut self.writer)?;
            default_value.serialize(self)?;
        }

        for directive in value.directives.iter() {
            directive.serialize(self)?;
        }

        Ok(())
    }

    fn serialize_type(&mut self, value: &Type) -> anyhow::Result<()> {
        match &value.base {
            BaseType::Named(name) => {
                name.serialize(self)?;
            }
            BaseType::List(list) => {
                self.formatter.begin_array(&mut self.writer)?;
                list.serialize(self)?;
                self.formatter.end_array(&mut self.writer)?;
            }
        }

        if !value.nullable {
            self.formatter
                .write_non_null_type_indicator(&mut self.writer)?;
        }

        Ok(())
    }

    fn serialize_fragment_definition(
        &mut self,
        name: &Name,
        value: &FragmentDefinition,
    ) -> anyhow::Result<()> {
        self.formatter.write_keyword(&mut self.writer, "fragment")?;

        self.formatter.write_separator(&mut self.writer)?;
        name.serialize(self)?;
        self.formatter.write_separator(&mut self.writer)?;

        value.type_condition.serialize(self)?;

        for directive in value.directives.iter() {
            directive.serialize(self)?;
        }

        self.formatter
            .after_operation_or_fragment_signature(&mut self.writer)?;

        value.selection_set.serialize(self)?;

        Ok(())
    }

    fn serialize_type_condition(&mut self, value: &TypeCondition) -> anyhow::Result<()> {
        self.formatter.write_keyword(&mut self.writer, "on")?;
        self.formatter.write_separator(&mut self.writer)?;
        value.on.serialize(self)
    }

    fn serialize_name(&mut self, value: &Name) -> anyhow::Result<()> {
        self.formatter
            .write_string_fragment(&mut self.writer, value)?;

        Ok(())
    }

    fn serialize_value(&mut self, value: &Value) -> anyhow::Result<()> {
        match value {
            Value::Null => {
                self.formatter.write_null(&mut self.writer)?;
            }
            Value::Number(value) => {
                value.serialize(self)?;
            }
            // TODO: Support string character escaping
            Value::String(value) => {
                self.formatter.begin_string(&mut self.writer)?;
                self.formatter
                    .write_string_fragment(&mut self.writer, value)?;
                self.formatter.end_string(&mut self.writer)?;
            }
            Value::Boolean(value) => {
                self.formatter.write_bool(&mut self.writer, *value)?;
            }
            Value::Variable(name) => {
                self.formatter.begin_variable(&mut self.writer)?;
                name.serialize(self)?;
            }
            Value::Enum(value) => {
                value.serialize(self)?;
            }
            Value::List(list) => {
                self.serialize_value_array(list)?;
            }
            Value::Object(value) => {
                self.serialize_value_object(value)?;
            }
            Value::Binary(value) => {
                self.formatter
                    .write_byte_array(&mut self.writer, &value[..])?;
            }
        }

        Ok(())
    }

    fn serialize_value_array(&mut self, value: &[Value]) -> anyhow::Result<()> {
        self.formatter.begin_array(&mut self.writer)?;

        let mut iter = value.iter().peekable();
        while let Some(value) = iter.next() {
            value.serialize(self)?;

            // If there are more items, add a separator
            if iter.peek().is_some() {
                self.formatter.write_item_separator(&mut self.writer)?;
            }
        }

        self.formatter.end_array(&mut self.writer)?;

        Ok(())
    }

    fn serialize_value_object(&mut self, value: &IndexMap<Name, Value>) -> anyhow::Result<()> {
        self.formatter.begin_object(&mut self.writer)?;

        let mut iter = value.iter().peekable();
        while let Some((key, value)) = iter.next() {
            key.serialize(self)?;
            self.formatter
                .write_name_value_separator(&mut self.writer)?;
            value.serialize(self)?;

            // If there are more items, add a separator
            if iter.peek().is_some() {
                self.formatter.write_item_separator(&mut self.writer)?;
            }
        }

        self.formatter.end_object(&mut self.writer)?;

        Ok(())
    }

    fn serialize_const_value(&mut self, value: &ConstValue) -> anyhow::Result<()> {
        match value {
            ConstValue::Null => {
                self.formatter.write_null(&mut self.writer)?;
            }
            ConstValue::Number(value) => {
                value.serialize(self)?;
            }
            ConstValue::String(value) => {
                // TODO: Support string character escaping
                self.formatter.begin_string(&mut self.writer)?;
                self.formatter
                    .write_string_fragment(&mut self.writer, value)?;
                self.formatter.end_string(&mut self.writer)?;
            }
            ConstValue::Boolean(value) => {
                self.formatter.write_bool(&mut self.writer, *value)?;
            }
            ConstValue::Enum(value) => {
                value.serialize(self)?;
            }
            ConstValue::List(list) => {
                self.serialize_const_value_array(list)?;
            }
            ConstValue::Object(object) => {
                self.serialize_const_value_object(object)?;
            }
            ConstValue::Binary(value) => {
                self.formatter
                    .write_byte_array(&mut self.writer, &value[..])?;
            }
        }

        Ok(())
    }

    fn serialize_const_value_array(&mut self, value: &[ConstValue]) -> anyhow::Result<()> {
        self.formatter.begin_array(&mut self.writer)?;

        let mut iter = value.iter().peekable();
        while let Some(value) = iter.next() {
            value.serialize(self)?;

            // If there are more items, add a separator
            if iter.peek().is_some() {
                self.formatter.write_item_separator(&mut self.writer)?;
            }
        }

        self.formatter.end_array(&mut self.writer)?;

        Ok(())
    }

    fn serialize_const_value_object(
        &mut self,
        value: &IndexMap<Name, ConstValue>,
    ) -> anyhow::Result<()> {
        self.formatter.begin_object(&mut self.writer)?;

        let mut iter = value.iter().peekable();
        while let Some((key, value)) = iter.next() {
            key.serialize(self)?;
            self.formatter
                .write_name_value_separator(&mut self.writer)?;
            value.serialize(self)?;

            // If there are more items, add a separator
            if iter.peek().is_some() {
                self.formatter.write_item_separator(&mut self.writer)?;
            }
        }

        self.formatter.end_object(&mut self.writer)?;

        Ok(())
    }

    fn serialize_number(&mut self, value: &Number) -> anyhow::Result<()> {
        if let Some(u) = value.as_u64() {
            self.formatter.write_u64(&mut self.writer, u)?;
        } else if let Some(i) = value.as_i64() {
            self.formatter.write_i64(&mut self.writer, i)?;
        } else if let Some(f) = value.as_f64() {
            self.formatter.write_f64(&mut self.writer, f)?;
        } else {
            unreachable!("invalid number")
        }

        Ok(())
    }
}

// Implement `AstSerialize` for a type that can be serialized.
macro_rules! impl_serialize {
    ($ty:ty, $method:ident) => {
        impl Serialize for $ty {
            #[inline]
            fn serialize<W, F>(&self, ser: &mut Serializer<W, F>) -> anyhow::Result<()>
            where
                W: io::Write,
                F: Formatter,
            {
                ser.$method(self)
            }
        }
    };
}

impl<T> Serialize for Positioned<T>
where
    T: Serialize,
{
    #[inline]
    fn serialize<W, F>(&self, ser: &mut Serializer<W, F>) -> anyhow::Result<()>
    where
        W: io::Write,
        F: Formatter,
    {
        self.node.serialize(ser)
    }
}

impl Serialize for OperationDefinition {
    #[inline]
    fn serialize<W, F>(&self, ser: &mut Serializer<W, F>) -> anyhow::Result<()>
    where
        W: io::Write,
        F: Formatter,
    {
        // If we are serializing an "operation definition" instance assume that there is a single
        // operation in the document
        ser.serialize_operation_definition(self, None, true)
    }
}

impl Serialize for (Name, OperationDefinition) {
    #[inline]
    fn serialize<W, F>(&self, ser: &mut Serializer<W, F>) -> anyhow::Result<()>
    where
        W: io::Write,
        F: Formatter,
    {
        let (name, operation) = self;
        ser.serialize_operation_definition(operation, Some(name), true)
    }
}

impl Serialize for (Name, FragmentDefinition) {
    #[inline]
    fn serialize<W, F>(&self, ser: &mut Serializer<W, F>) -> anyhow::Result<()>
    where
        W: io::Write,
        F: Formatter,
    {
        let (name, fragment) = self;
        ser.serialize_fragment_definition(name, fragment)
    }
}

impl_serialize!(ExecutableDocument, serialize_executable_document);
impl_serialize!(SelectionSet, serialize_selection_set);
impl_serialize!(Selection, serialize_selection);
impl_serialize!(Field, serialize_selection_field);
impl_serialize!(FragmentSpread, serialize_fragment_spread);
impl_serialize!(InlineFragment, serialize_inline_fragment);
impl_serialize!(Directive, serialize_directive);
impl_serialize!(VariableDefinition, serialize_variable_definition);
impl_serialize!(Type, serialize_type);
impl_serialize!(ConstValue, serialize_const_value);
impl_serialize!(TypeCondition, serialize_type_condition);
impl_serialize!(Name, serialize_name);
impl_serialize!(Value, serialize_value);
impl_serialize!(Number, serialize_number);
