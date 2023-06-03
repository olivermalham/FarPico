use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};


pub trait HalComponent {
    fn dispatch(&self, action: &str);
}

#[derive(Deserialize, Debug)]
pub struct ActionString {
    pub action: String,
    pub parameters: Map<String, Value>
}

#[derive(Serialize)]
pub struct Gpio {
    #[serde(skip_serializing)]
    pub pin: u8,  // TODO: This field should be a Pin struct from the microcontroller HAL!
    pub state: bool,
    pub actions: Vec<String>
}

impl HalComponent for Gpio {
    fn dispatch(&self, action_string: &str){
        println!("GPIO Action {}", action_string);
        let action: ActionString = serde_json::from_str(action_string).unwrap();
        println!("ActionString object: {:?}", action);
    }
}

pub trait HalFuncs {
    // Build JSON state string here - implemented as a trait function to allow additional logic
    // over just using Serde on the struct directly.
    fn to_json(&self) -> String;

    // Dispatch an action string received from the client - JSON formatted as per FarPi-Server
    fn dispatch(&self, action: &str);
}

