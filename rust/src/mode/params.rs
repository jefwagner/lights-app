use serde::{Serialize, Deserialize};

use crate::lights::LedColor;

/// An input or adjustable parameter for a lights mode
#[derive(Clone, Debug, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    fn foo() {
        let x = Param{ 
            name: "foo".into(),
            value: Value::Range(50),
            meta: Some(Meta::Range { min: 0, max: 100 }),
        };
        eprintln!("{}", serde_json::to_string(&x).unwrap());
        assert!(false);
    }

    #[test]
    fn bar() {
        let x = r#" {"name":"bar", "type": "range", "value":50}"#;
        let y: Param = serde_json::from_str(x).unwrap();
        eprintln!("{y:?}");
        assert!(false);
    }

    #[test]
    fn baz() {
        let x = Param{ 
            name: "baz".into(),
            value: Value::Color(LedColor { r: 255, g: 127, b: 0 }),
            meta: Some(Meta::Color),
        };
        eprintln!("{}", serde_json::to_string(&x).unwrap());
        assert!(false);
    }

    #[test]
    fn buz() {
        let x = r##"{"name":"bar", "type": "color", "value":"#123456"}"##;
        let y: Param = serde_json::from_str(x).unwrap();
        eprintln!("{y:?}");
        assert!(false);
    }

}