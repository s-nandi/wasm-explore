mod bindings;

use crate::bindings::exports::component::library1::greeter::{Greeting, Guest};

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> Greeting {
        let text = "Hello, World!".to_string();
        let times = vec![3, 4, 5];
        Greeting { text, times }
    }
}
