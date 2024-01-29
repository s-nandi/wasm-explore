mod bindings;

use crate::bindings::component::library1::greeter::{hello_world, Greeting};
use crate::bindings::component::library2::generator::{generate, generate_many, generate_paths};

fn main() {
    println!("In main (app/main.rs)");
    let Greeting { text, times } = hello_world();
    let generated_paths = generate_paths();

    for path in generated_paths {
        let generated_value = generate();
        let generated_values = generate_many().into_iter().collect::<Vec<i32>>();

        println!("Path {}:", path.name());
        println!(
            "{:>20} @ {:?} ({} | {:?})",
            text, times, generated_value, generated_values
        );
    }
}
