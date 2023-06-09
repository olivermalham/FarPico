use serde::{Deserialize, Serialize};
use crate::hal::*;


// TODO! Console should be much more powerful that this

#[derive(Serialize)]
pub struct HalConsole {
    pub text: Vec<String> // Line buffer
}

#[derive(Deserialize, Debug)]
struct HalConsoleParamaters {
    pub _clear: i32
}

impl HalComponent for HalConsole {
    fn dispatch(&mut self, action: &str, parameter_json: &str) -> Result <(), String>{

        println!("HalConsole Action {} - {:?}", action, parameter_json);
        // TODO: Many more actions to add here, although doubt I can easily get the same range of
        // TODO: functionality as the Python version due to lack of introspection
        match action {
            "action_clear" => {
                let _parameters: HalConsoleParamaters = serde_json::from_str(parameter_json).unwrap();
                self.text.clear();
                Ok(())
            },
            _ => Err("Buggered!".to_string()) // FIXME!
        }
    }

    fn refresh(&mut self) -> Result<(), String> {
        return Ok(());
    }
}
