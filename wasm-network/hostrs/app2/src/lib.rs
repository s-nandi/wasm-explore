mod bindings;

use crate::bindings::component::dependency::imports::myimport;
use bindings::Guest;

struct Component;

impl Guest for Component {
    fn hello_world() -> String {
        let v = myimport();
        let name = v.name();
        println!("Hello world side effect");

        format!("Hello world with import ({name})")
    }
}
