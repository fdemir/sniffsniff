use std::net::IpAddr;
use std::str::FromStr;

const DEFAULT_THREADS: u16 = 4;

pub struct Arguments {
    // v4 or v6
    pub ipaddr: IpAddr,
    pub threads: u16,
}

pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
    if args.len() > 4 {
        return Err("not enough arguments");
    }

    let f = args[1].clone();

    if let Ok(ipaddr) = IpAddr::from_str(&f) {
        return Ok(Arguments {
            ipaddr,
            threads: DEFAULT_THREADS,
        });
    }

    let flag = args[1].clone();

    match flag.as_str() {
        "-j" => {
            let ipaddr = IpAddr::from_str(&args[3]).unwrap();

            let threads = match args[2].parse::<u16>() {
                Ok(threads) => threads,
                Err(_) => return Err("invalid number of threads!"),
            };

            return Ok(Arguments { ipaddr, threads });
        }
        "-h" | "--help" => {
            return Err("help");
        }
        _ => {
            return Err("");
        }
    };
}
