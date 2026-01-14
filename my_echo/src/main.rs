use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: my_echo <message>".into());
    }
    let message = args[1..].join(" ");
    println!("{}", message);
    Ok(())
}