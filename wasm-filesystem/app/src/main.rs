mod bindings;

use std::fs::File;
use std::io::Write;

fn main() {
    println!("Hello, world!");
    let mut file = File::create("bin/bar.txt").expect("could not create file");
    // This next line would fail:
    // let mut file = File::create("./foo.txt").expect("could not create file");
    println!("Created file");
    file.write_all(b"Hello, world!")
        .expect("could not write to file");
    println!("Wrote file!");
}
