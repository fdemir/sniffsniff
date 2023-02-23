use std::env;

mod arguments;
mod scan;
mod spawn;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = arguments::new(&args).unwrap_or_else(|err| {
        match err {
            "help" => println!("Usage: -j <threads> <ipaddr> <port>"),
            _ => println!("Problem parsing arguments: {}", err),
        }
        std::process::exit(1);
    });

    let open_ports = spawn::init(arguments);

    println!("");
    println!("Open ports: ({} total)", open_ports.len());

    for p in open_ports {
        println!("{}", p);
    }
}
