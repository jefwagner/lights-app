use serde::{Serialize, Deserialize};

use crate::lights::LedColor;

/// An input or adjustable parameter for a lights mode
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Param{
    /// The parameter name
    pub name: String,
    /// The parameter value
    #[serde(flatten)]
    pub value: Value,
    /// The metadata needed to render the parameter widget
    pub meta: Option<Meta>,
}

/// The parameter value that returned by the web-app front-end
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", tag="type", content="value")]
pub enum Value{
    /// A sliding on-off toggle
    Toggle(bool),
    /// A button
    Button,
    /// A slider that selects a value from a range
    Range(isize),
    /// A color selector
    Color(LedColor),
}

/// The parameters metadata used by the front-end to render the widget
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Meta {
    /// A sliding on-off toggle
    Toggle{ on: String, off: String },
    /// A button
    Button{ label: String },
    /// A slider that selects a value from a range
    Range{min: isize, max: isize},
    /// A color selector
    Color,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    // quick tests with 'partial compile' that lets me see what's going on
    #[test]
    fn param_serialize() {
        let x = Param{ 
            name: "foo".into(),
            value: Value::Range(50),
            meta: Some(Meta::Range { min: 0, max: 100 }),
        };
        let expected = r#"{"name":"foo","type":"range","value":50,"meta":{"min":0,"max":100}}"#;
        assert_eq!(
            serde_json::to_string(&x).unwrap().as_str(),
            expected
        );
    }

    #[test]
    fn param_deserialize() {
        let expected = Param {
            name: "bar".into(),
            value: Value::Range(50),
            meta: None,
        };
        let input_str = r#" {"name":"bar", "type": "range", "value":50}"#;
        let input: Param = serde_json::from_str(input_str).unwrap();
        assert_eq!(
            input,
            expected
        );
    }

    #[test]
    fn param_serialize_color() {
        let x = Param{ 
            name: "baz".into(),
            value: Value::Color(LedColor { r: 255, g: 127, b: 0 }),
            meta: Some(Meta::Color),
        };
        let expected = r##"{"name":"baz","type":"color","value":"#ff7f00","meta":null}"##;
        assert_eq!(
            serde_json::to_string(&x).unwrap().as_str(),
            expected
        );
    }

    #[test]
    fn parame_deserialize_color() {
        let expected = Param { 
            name: "bar".into(), 
            value: Value::Color(LedColor { r: 18, g: 52, b: 86 }), 
            meta: None 
        };
        let input_str = r##"{"name":"bar", "type": "color", "value":"#123456"}"##;
        let input: Param = serde_json::from_str(input_str).unwrap();
        assert_eq!(
            input,
            expected
        );
    }

}