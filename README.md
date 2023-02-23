# SniffSniff

I am trying to learn Rust programming language. Here is a small project that sniffs the ports of a given host. I want to give some info what is port, tcp and host.

**port:** a number that is used to identify a specific process to which an incoming network message is to be forwarded to.

**tcp:** a protocol that is used to establish a connection between two hosts.

**host:** a computer that is connected to a network.

### What is port sniffing anyway?

Port sniffing is a technique that is used to scan the ports of a host. It is used to find out which ports are open and which are closed. It is also used to find out which services are running on a host.

### How does it work?

The program sends a TCP SYN packet to the host and waits for a response. If the port is open, the host will send a TCP SYN/ACK packet. If the port is closed, the host will send a TCP RST packet. ....

Diagram:

### Things that i have learned so far

- unwrap: unwrap is used to get the value of a Result. If the Result is an Err, unwrap will panic. it is good if you dont want to handle the error.
- move: simply move ownership into a closure.
- match: works like switch case in other languages.
- drop: dispose of a value. 
- &'static str: a string that lives for the entire duration of the program.
- io::stdout().flush(): flushes the buffer of the stdout.
- thread::spawn: spawns a new thread.
- afunc()?: the ? means that if the result of afunc() is an Err, the whole function will return the Err value.

## Usage
```
cargo run -- -j <threads> <ipaddr>
```

## Example
```
cargo run -- -j 10000 192.168.1.1
```
```bash
....
Open ports: (4 total)
53
80
52869
443
```
## Build
```
cargo build
```
## Run
```
cargo run
```

## License
Licensed under the MIT license. See the LICENSE file for details.