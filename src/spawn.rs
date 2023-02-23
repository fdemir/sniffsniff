use std::sync::mpsc::channel;
use std::thread;

use crate::arguments;
use crate::scan;

pub fn init(arguments: arguments::Arguments) -> Vec<u16> {
    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();

    for i in 0..num_threads {
        // bind to another tx var, this way each thread has its own tx(transmitter)
        let tx = tx.clone();

        thread::spawn(move || {
            scan::start(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    // drop the tx, this way the rx will know that there are no more tx
    drop(tx);

    for p in rx {
        out.push(p);
    }

    return out;
}
