use serde::Serialize;
use crate::hal::*;
use crate::hal_gpio::Gpio;

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

    // FIXME: This should really be a default implementation, but can't use Serialise on a trait
    fn to_json(&self) -> String {
        serde_json::to_string(self).ok().unwrap()
    }

    fn dispatch(&mut self, target: &str, action: &str, parameter_json: &str) {
        println!("Action Request {}.{} - {}", target, action, parameter_json);
        // FIXME! This should return OK/Error type
        match target {
            "bcm00" => self.bcm00.dispatch(action, parameter_json),
            "bcm01" => self.bcm01.dispatch(action, parameter_json),
            "bcm02" => self.bcm02.dispatch(action, parameter_json),
            "bcm03" => self.bcm03.dispatch(action, parameter_json),
            _ => ()
        };
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
