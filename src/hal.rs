use serde::Serialize;


pub trait HalComponent {
    fn dispatch(&self, action: &String);
}

#[derive(Serialize)]
pub struct Gpio {
    #[serde(skip_serializing)]
    pub pin: u8,  // TODO: This field should be a Pin struct from the microcontroller HAL!
    pub state: bool,
    pub actions: Vec<String>
}

impl HalComponent for Gpio {
    fn dispatch(&self, action_string: &String){
        println!("GPIO Action {}", action_string);
    }
}

pub trait HalFuncs {
    fn to_json(&self) -> String;
    fn dispatch(&self, component: &String, action: &String, args: &Vec<String>);
}
