mod bindings;

use crate::bindings::exports::wasi::cli::run::Guest;

struct Component;

impl Guest for Component {
    fn run() -> Result<(), ()> {
        println!("Done!");
        Ok(())
    }
}
