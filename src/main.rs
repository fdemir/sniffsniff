use std::io::{self, Write};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::{env, net::IpAddr, net::TcpStream};

const DEFAULT_THREADS: u16 = 4;
// ??????
const MAX_PORT: u16 = u16::MAX;

struct Arguments {
    flag: String,
    // v4 or v6
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    // ok is arguments, reason we are using static is that we can send back the errs to the main funciton. @see borrow checker and lifetimes
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        // destruct ip address
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: DEFAULT_THREADS,
            });
        } else {
            let flag = args[1].clone();

            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = IpAddr::from_str(&args[3]).unwrap();

                let threads = match args[2].parse::<u16>() {
                    Ok(threads) => threads,
                    Err(_) => return Err("invalid number of threads"),
                };

                return Ok(Arguments {
                    flag,
                    ipaddr,
                    threads,
                });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;

    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        // This code is used to check if the number of threads that will be created is more than the number of ports available
        // This is done by subtracting the port number from the maximum number of ports
        // If the result is less than the number of threads, then the number of threads is greater than the number of ports available
        // In this case, we break out of the loop

        if (MAX_PORT - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err == "help" {
            println!("Usage: -j <threads> <ipaddr> <port>");
        } else {
            println!("Problem parsing arguments: {}", err);
        }
        std::process::exit(1);
    });

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        // bind to another tx var, this way each thread has its own tx(transmitter)
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    // drop the tx, this way the rx will know that there are no more tx
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    for v in out {
        println!("{} is open", v);
    }
}
