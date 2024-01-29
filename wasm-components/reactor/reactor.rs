use std::io::{self, BufRead};

fn dispatch(s: String) {
    std::println!("{}", s);
}

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();

    while let Some(line) = lines.next() {
        let input = line.unwrap();

        if input.len() == 0 {
            break;
        }
        dispatch(input);
    }
    Ok(())
}
