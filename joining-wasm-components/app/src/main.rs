mod bindings;

use crate::bindings::component::library1::greeter::{hello_world, Greeting};
use crate::bindings::component::library2::generator::{generate, generate_many};

fn main() {
    println!("In main");
    let Greeting { text, times } = hello_world();
    let generated_value = generate();
    let generated_values = generate_many().into_iter().collect::<Vec<i32>>();
    println!(
        "{} @ {:?} ({} | {:?})",
        text, times, generated_value, generated_values
    );
}
