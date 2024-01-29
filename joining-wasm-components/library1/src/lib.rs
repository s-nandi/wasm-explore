mod bindings;

use crate::bindings::exports::component::library1::greeter::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}
