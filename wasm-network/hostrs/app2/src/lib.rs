mod bindings;

use crate::bindings::component::dep::dep::myimport;
use bindings::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        let v = myimport();

        format!("Hello world with import ({v})")
    }
}
