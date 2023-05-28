use std::collections::HashMap;
use serde::Serialize;


/*
    So, how to represent the hardware in a way that can be easily serialised into JSON, and can
    support the simple RPC mechanism used by FarPi? What is the Rust way of doing things? My Python
    implementation is heavily object orientated, and makes heavy use of Python's introspection
    features to make it as easy as possible to create a new HAL for a new project.

    I want to make it as easy as possible to use FarPico to far new projects, but I need to bear in
    mind the requirement for a low resource footprint as well. Plus, I want it to follow Rust best
    practice / idioms as well.

    Makes sense to use Serde for serialising a state structure into JSON. Needs to be compatible
    with the FarPi-Server protocols so the same client code can work with both. So that means a
    nested struct for storing the current state.

    State structs only need to be serializable, state is never sent to the server from the client,
    actions are, which are much simpler.

    Application specific function will populate the hal structure after an empty one is created,
    so there should be no need to mess with the code in here unless adding a new component type.

    Each component type should have a method for handling action strings. That method will decide
    what to do for any given action string. When dispatching, loop through each hashmap looking for
    the component name. Once found, call the action handled with the request arguments. Assumes that
    component names are unique! (Which they are in FarPi Server as they map directly to the HAL object
    scope).
*/


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

/*
    HAL structure. Use lists of each component type
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

pub trait HalFuncs {
    fn to_json(&self) -> String;
    fn dispatch(&self, component: &String, action: &String, args: &Vec<String>);
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
