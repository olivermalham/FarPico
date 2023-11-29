use serde::Serialize;
use crate::hal::*;
use crate::hal_gpio::Gpio;
use crate::hal_console::HalConsole;
use farpi_macros_lib::HAL;

/*
    Very basic example HAL
    All members of a HAL must implement the HalComponent trait!
 */
#[derive(Serialize, HAL)]
pub struct ExampleHal {
    pub bcm00: Gpio,
    pub bcm01: Gpio,
    pub bcm02: Gpio,
    pub bcm03: Gpio,
    pub error: HalError,
    pub message: HalConsole,
}


pub fn build_hal() -> ExampleHal {
    println!("Building example HAL");
    ExampleHal {
        bcm00: Gpio{pin:0x01, state:false, actions:vec!["bcm00.action_set".to_string(), "bcm00.action_toggle".to_string()]},
        bcm01: Gpio{pin:0x02, state:false, actions:vec!["bcm01.action_set".to_string(), "bcm01.action_toggle".to_string()]},
        bcm02: Gpio{pin:0x03, state:false, actions:vec!["bcm02.action_set".to_string(), "bcm02.action_toggle".to_string()]},
        bcm03: Gpio{pin:0x04, state:false, actions:vec!["bcm03.action_set".to_string(), "bcm03.action_toggle".to_string()]},
        error: HalError{text: "No error here!".to_string()},
        message: HalConsole{text: "Using ExampleHAL".to_string()},
    }
}
