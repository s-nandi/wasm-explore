mod bindings;

use crate::bindings::component::library1::greeter::{hello_world, Greeting};

fn main() {
    println!("In main");
    let Greeting { text, times } = hello_world();
    println!("{} @ {:?}", text, times);
}
