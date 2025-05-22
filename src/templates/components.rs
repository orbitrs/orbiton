use liquid::model::{ArrayView, DisplayCow, ObjectView, ScalarCow, Value, ValueView};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropType {
    String,
    Number,
    Boolean,
    Array(Box<PropType>),
    Object(HashMap<String, PropType>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub template: String,
    pub props: HashMap<String, PropType>,
    pub description: String,
}

#[allow(dead_code)]
impl Component {
    pub fn new(
        name: String,
        template: String,
        props: HashMap<String, PropType>,
        description: String,
    ) -> Self {
        Self {
            name,
            template,
            props,
            description,
        }
    }

    pub fn add_prop(&mut self, name: String, prop_type: PropType) {
        self.props.insert(name, prop_type);
    }

    pub fn generate_code(&self) -> String {
        // Basic implementation - can be enhanced
        format!(
            "Component {{ name: {}, props: [{}] }}",
            self.name,
            self.props.keys().cloned().collect::<Vec<_>>().join(", ")
        )
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.description)
    }
}

impl ValueView for Component {
    fn as_debug(&self) -> &dyn Debug {
        self
    }

    fn render(&self) -> DisplayCow<'_> {
        DisplayCow::Owned(Box::new(self.to_string()))
    }

    fn source(&self) -> DisplayCow<'_> {
        DisplayCow::Owned(Box::new(self.to_string()))
    }

    fn type_name(&self) -> &'static str {
        "component"
    }

    fn query_state(&self, state: liquid::model::State) -> bool {
        match state {
            liquid::model::State::Truthy => true,
            liquid::model::State::DefaultValue => false,
            liquid::model::State::Empty => false,
            _ => false,
        }
    }

    fn to_kstr(&self) -> liquid::model::KStringCow<'_> {
        liquid::model::KStringCow::from_string(self.to_string())
    }

    fn to_value(&self) -> Value {
        Value::Nil
    }

    fn as_object(&self) -> Option<&dyn ObjectView> {
        None
    }

    fn as_array(&self) -> Option<&dyn ArrayView> {
        None
    }

    fn as_scalar(&self) -> Option<ScalarCow<'_>> {
        Some(ScalarCow::new(self.name.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_component() {
        let mut component = Component::new(
            "Button".to_string(),
            "button".to_string(),
            HashMap::new(),
            "A simple button component".to_string(),
        );
        component.add_prop("disabled".to_string(), PropType::Boolean);
        component.add_prop("label".to_string(), PropType::String);

        let code = component.generate_code();
        assert!(code.contains("disabled"));
        assert!(code.contains("label"));
    }

    #[test]
    fn test_complex_component() {
        let mut user_type = HashMap::new();
        user_type.insert("name".to_string(), PropType::String);
        user_type.insert("age".to_string(), PropType::Number);

        let mut component = Component::new(
            "UserCard".to_string(),
            "user-card".to_string(),
            HashMap::new(),
            "A user card component".to_string(),
        );
        component.add_prop(
            "tags".to_string(),
            PropType::Array(Box::new(PropType::String)),
        );
        component.add_prop("user".to_string(), PropType::Object(user_type));

        let code = component.generate_code();
        assert!(code.contains("tags"));
        assert!(code.contains("user"));
    }
}
