mod bindings;

use crate::bindings::component::library1::greeter::hello_world;

fn main() {
    println!("In main");
    println!("{}", hello_world());
}
