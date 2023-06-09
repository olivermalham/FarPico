use serde::Serialize;

/*
    Core functionality that must be implemented by Hardware Abstraction Layers (HALs).
    A HAL implements the concrete interface to the project hardware.
 */

// Every HAL component must implement these methods
pub trait HalComponent {
    fn dispatch(&mut self, action: &str, parameter_json: &str) -> Result <(), String>;
    fn refresh(&mut self) -> Result <(), String>;
}

// HalFuncs are usually implemented via derive macro
pub trait HalFuncs {
    fn to_json(&self) -> String;
    fn dispatch(&mut self, target: &str, action: &str, parameter_json: &str) -> Result <(), String>;
    fn refresh(&mut self) -> Result <(), String>;
}


/*
    Basic components that all HAL implementations are likely to need
*/

#[derive(Serialize)]
pub struct HalError {
    pub text: Vec<String> // Line buffer
}


impl HalComponent for HalError {
    fn dispatch(&mut self, action: &str, _: &str) -> Result <(), String>{

        println!("HalError Clear Action");

        match action {
            "action_clear" => {
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

impl HalError {
    fn add(&mut self, msg: String){
        self.text.push(msg);
    }
}