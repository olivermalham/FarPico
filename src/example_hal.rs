use serde::Serialize;
use crate::hal::*;

/*
    Example HAL
 */
#[derive(Serialize)]
pub struct ExampleHal {
    pub bcm00: Gpio,
    pub bcm01: Gpio,
    pub bcm02: Gpio,
    pub bcm03: Gpio,
    pub error: String,
    pub message: String,
    pub cycle: i32
}

impl HalFuncs for ExampleHal {

    fn to_json(&self) -> String {
        // Build JSON state string here
        serde_json::to_string(self).ok().unwrap()
    }

    fn dispatch(&self, component: &String, action: &String, args: &Vec<String>) {
        todo!("Add Result() for returning error / success etc")
    }
}

pub fn build_hal() -> ExampleHal {
    println!("Building example HAL");
    ExampleHal {
        bcm00: Gpio{pin:0x01, state:false, actions:vec!["bcm00.action_set".to_string(), "bcm00.action_toggle".to_string()]},
        bcm01: Gpio{pin:0x02, state:false, actions:vec!["bcm01.action_set".to_string(), "bcm01.action_toggle".to_string()]},
        bcm02: Gpio{pin:0x03, state:false, actions:vec!["bcm02.action_set".to_string(), "bcm02.action_toggle".to_string()]},
        bcm03: Gpio{pin:0x04, state:false, actions:vec!["bcm03.action_set".to_string(), "bcm03.action_toggle".to_string()]},
        error: "No error here!".to_string(),
        message: "Using ExampleHAL".to_string(),
        cycle: 0
    }
}
