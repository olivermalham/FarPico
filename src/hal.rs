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

    Use one hashmap per type of component supported. FarPi doesn't support nested components in the
    HAL, so that should be OK. Serialising the entire state would mean serialising the dict for
    each component type, then combining the json dicts and adding the HAL level fields.

    Application specific function will populate the hal structure after an empty one is created,
    so there should be no need to mess with the code in here unless adding a new component type.

    Each component type should have a method for handling action strings. That method will decide
    what to do for any given action string. When dispatching, loop through each hashmap looking for
    the component name. Once found, call the action handled with the request arguments. Assumes that
    component names are unique! (Which they are in FarPi Server as they map directly to the HAL object
    scope).

*/


#[derive(Serialize)]
pub struct Gpio {
    pin: u8,
    value: bool
}

impl Gpio {
    pub fn action(action_string: &String){
        println!("GPIO Action {}", action_string);
    }
}

/*
    HAL structure. Use lists of each component type
 */
pub struct Hal {
    gpio: HashMap<String, Gpio>,
    error: String,
    message: String
}

impl Hal {

    pub fn new() -> Hal {
        Hal {
            gpio: HashMap::new(),
            error: "".to_string(),
            message: "".to_string()
        }
    }

    pub fn to_json(&self) -> String {
        // Build JSON state string here
        "Wibble".to_string()
    }

}
