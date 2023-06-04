

pub trait HalComponent {
    // FIXME! This should return an Ok/Error enum
    fn dispatch(&mut self, action: &str, parameter_json: &str);
}

pub trait HalFuncs {
    // Build JSON state string here - implemented as a trait function to allow additional logic
    // over just using Serde on the struct directly.
    fn to_json(&self) -> String;

    // Dispatch an action string received from the client - JSON formatted as per FarPi-Server
    // FIXME! This should return an Ok/Error enum
    fn dispatch(&mut self, target: &str, action: &str, parameter_json: &str);
}

