use serde::{Deserialize, Serialize};

use crate::lights::Param;

/// The application state for the lights controller
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    on_off: bool,
    modes: Vec<String>,
    selected: usize,
    params: Vec<Param>
}

impl AppState {
    pub fn init() -> Self {
        AppState {
            on_off: false,
            modes: vec![],
            selected: 0,
            params: vec![],
        }
    }

    pub fn new(on_off: bool, modes: Vec<String>, selected: usize, params: Vec<Param>) -> Self {
        AppState{ on_off, modes, selected, params}
    }
}

/// The command sent form the front-end to backend
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AppStateChange {
    OnOff(bool),
    ModeSelect(usize),
    ChangeParam(Param),
    Stop,
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::lights::{
        LedColor,
        Param,
        params::{
            Value,
            Meta,
        },
    };
    use serde_json;

    fn foo_mode() -> (String, Vec<Param>) {
        (
            "foo".to_string(),
            vec![
                Param {
                    name: "r".into(),
                    value: Value::Range(255),
                    meta: Some(Meta::Range { min: 0, max: 255 }),
                },
                Param {
                    name: "g".into(),
                    value: Value::Range(0),
                    meta: Some(Meta::Range { min: 0, max: 255 }),
                },
                Param {
                    name: "b".into(),
                    value: Value::Range(0),
                    meta: Some(Meta::Range { min: 0, max: 255 }),
                }
            ]
        )
    }

    fn bar_mode() -> (String, Vec<Param>) {
        (
            "bar".to_string(),
            vec![
                Param {
                    name: "Switch".into(),
                    value: Value::Toggle(false),
                    meta: Some(Meta::Toggle { on: "On".into(), off: "Off".into() })
                },
                Param {
                    name: "color".into(),
                    value: Value::Color(LedColor { r: 255, g: 0, b: 0 }),
                    meta: Some(Meta::Color),
                }
            ]
        )
    }

    fn baz_mode() -> (String, Vec<Param>) {
        (
            "baz".to_string(),
            vec![
                Param {
                    name: "Button".into(),
                    value: Value::Button,
                    meta: Some(Meta::Button { label: "Press me!".into() })
                },
            ]
        )
    }

    #[test]
    fn serialize_foo() {
        let mut modes = vec![];
        let (name, params) = foo_mode();
        modes.push(name);
        let (name, _) = bar_mode();
        modes.push(name);
        let (name, _) = baz_mode();
        modes.push(name);

        let app_state = AppState{
            on_off: false,
            modes: modes,
            selected: 0,
            params: params
        };
        let expected = r##"{"onOff":false,"modes":["foo","bar","baz"],"selected":0,"params":[{"name":"r","type":"range","value":255,"meta":{"min":0,"max":255}},{"name":"g","type":"range","value":0,"meta":{"min":0,"max":255}},{"name":"b","type":"range","value":0,"meta":{"min":0,"max":255}}]}"##;
        assert_eq!(
            serde_json::to_string(&app_state).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_bar() {
        let mut modes = vec![];
        let (name, _) = foo_mode();
        modes.push(name);
        let (name, params) = bar_mode();
        modes.push(name);
        let (name, _) = baz_mode();
        modes.push(name);

        let app_state = AppState{
            on_off: false,
            modes: modes,
            selected: 1,
            params: params
        };
        let expected = r##"{"onOff":false,"modes":["foo","bar","baz"],"selected":1,"params":[{"name":"Switch","type":"toggle","value":false,"meta":{"on":"On","off":"Off"}},{"name":"color","type":"color","value":"#ff0000","meta":null}]}"##;
        assert_eq!(
            serde_json::to_string(&app_state).unwrap(),
            expected
        );
    }

    #[test]
    fn serialize_baz() {
        let mut modes = vec![];
        let (name, _) = foo_mode();
        modes.push(name);
        let (name, _) = bar_mode();
        modes.push(name);
        let (name, params) = baz_mode();
        modes.push(name);

        let app_state = AppState{
            on_off: false,
            modes: modes,
            selected: 1,
            params: params
        };
        let expected = r##"{"onOff":false,"modes":["foo","bar","baz"],"selected":1,"params":[{"name":"Button","type":"button","meta":{"label":"Press me!"}}]}"##;
        assert_eq!(
            serde_json::to_string(&app_state).unwrap(),
            expected
        );
    }

    #[test]
    fn on_off() {
        let input_str = r##"{"onOff":true}"##;
        let input: AppStateChange = serde_json::from_str(input_str).unwrap();
        let expected = AppStateChange::OnOff(true);
        assert_eq!(input, expected);
    }

    #[test]
    fn mode_select() {
        let input_str = r##"{"modeSelect":0}"##;
        let input: AppStateChange = serde_json::from_str(input_str).unwrap();
        let expected = AppStateChange::ModeSelect(0);
        assert_eq!(input, expected);
    }

    #[test]
    fn stop() {
        let input_str = r##""stop""##;
        let input: AppStateChange = serde_json::from_str(input_str).unwrap();
        let expected = AppStateChange::Stop;
        assert_eq!(input, expected);
    }

    #[test]
    fn change_param() {
        let input_str = r##"{"changeParam":{"name":"foo","type":"range","value":50}}"##;
        let input: AppStateChange = serde_json::from_str(input_str).unwrap();
        let expected = AppStateChange::ChangeParam(Param { name: "foo".into(), value: Value::Range(50), meta: None });
        assert_eq!(input, expected);
    }

}