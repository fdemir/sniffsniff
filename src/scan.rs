use std::io::{self, Write};
use std::sync::mpsc::Sender;
use std::{net::IpAddr, net::TcpStream};

const MAX_PORT: u16 = u16::MAX;

/**
 * Scan the ports
 */
pub fn start(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
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
