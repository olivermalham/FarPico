use serde::{Deserialize, Serialize};
use crate::hal::*;


#[derive(Serialize)]
pub struct Gpio {
    #[serde(skip_serializing)]
    pub pin: u8,  // TODO: This field should be a Pin struct from the microcontroller HAL!
    pub state: bool,
    pub actions: Vec<String>
}

#[derive(Deserialize, Debug)]
struct GpioSetParamaters {
    pub value: i32
}


impl HalComponent for Gpio {
    fn dispatch(&mut self, action: &str, parameter_json: &str){
        // FIXME! This should return an Ok/Error enum
        let parameters : GpioSetParamaters = serde_json::from_str(parameter_json).unwrap();
        println!("GPIO Action {} - {:?}", action, parameters);
        self.state = parameters.value != 0;
    }
}
