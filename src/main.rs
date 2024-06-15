mod client;
mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [server|client]", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "server" => {
            server::main()
        }
        "client" => {
            client::main()
        }
        _ => {
            eprintln!("Unknown argument: {}", args[1]);
            std::process::exit(1);
        }
    }
}
