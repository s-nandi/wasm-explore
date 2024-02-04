mod bindings;

use bindings::Guest;

use crate::bindings::component::host::network_provider;

struct Component;

impl Guest for Component {
    fn run(uri: String, filename: String) {
        network_provider::get(&uri, &filename);
    }
}
