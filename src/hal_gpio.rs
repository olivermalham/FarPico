use serde::{Deserialize, Serialize};
use crate::hal::*;


#[derive(Serialize)]
pub struct Gpio {
    #[serde(skip_serializing)]
    pub pin: u8,  // TODO: This field should be a Pin struct from the microcontroller HAL!
    pub state: bool,
    pub actions: Vec<String> // FIXME? Don't think this is needed
}

#[derive(Deserialize, Debug)]
struct GpioSetParamaters {
    pub value: i32
}


impl HalComponent for Gpio {
    fn dispatch(&mut self, action: &str, parameter_json: &str) -> Result <(), String>{

        println!("GPIO Action {} - {:?}", action, parameter_json);

        match action {
            "action_set" => {
                let parameters: GpioSetParamaters = serde_json::from_str(parameter_json).unwrap();
                self.state = parameters.value != 0;
                Ok(())
            },
            "action_toggle" => {
                self.state = !self.state;
                Ok(())
            },
            _ => Err("Buggered!".to_string()) // FIXME!
        }
    }

    // FIXME!
    fn refresh(&mut self) -> Result<(), String> {
        // TODO: Pull the current state from the GPIO pin
        return Ok(());
    }
}
