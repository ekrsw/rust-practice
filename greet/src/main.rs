use std::io;
use std::env;

fn main() {
    let args = env::args().skip(1);

    let name = if let Some(name) = args.next() {
        name
    } else {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        
        input.trim().to_string()
    };

    if name.is_empty() {
        eprintln!("Name is empty.");
        std::process::exit(1);
    }

    println!("Hello, {}!", name);
}