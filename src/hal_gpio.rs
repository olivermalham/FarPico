use serde::{Deserialize, Serialize};
use serde_json::error::Error;
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
    fn dispatch(&mut self, action: &str, parameter_json: &str) -> Result <(), serde_json::error::Error>{

        println!("GPIO Action {} - {:?}", action, parameter_json);

        match action {
            "action_set" => {
                let parameters: GpioSetParamaters = serde_json::from_str(parameter_json)?;
                self.state = parameters.value != 0;
                Ok(())
            },
            "action_toggle" => {
                self.state = !self.state;
                Ok(())
            },
            _ => Err() // FIXME!
        }
    }
}
