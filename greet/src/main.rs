use std::env;

fn main() -> Result<(), String>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Usage: greet <name>".into());
    } else {
        let name = &args[1];
        println!("Hello, {}!", name);
        Ok(())
    }
}