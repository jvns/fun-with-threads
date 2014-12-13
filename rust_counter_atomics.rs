use std::sync::Arc;
use std::sync::atomic::{AtomicUint, SeqCst};

const NUM_THREADS: uint = 20;
const NUM_INCREMENTS: uint = 100000u;

fn main() {
    let counter = Arc::new(AtomicUint::new(0));
    let (tx, rx) = channel();
    for _ in range(0u, NUM_THREADS) {
        let (counter, tx) = (counter.clone(), tx.clone());
        spawn(proc() {
            for _ in range(0u, NUM_INCREMENTS) {
                let value = counter.load(SeqCst);
                counter.fetch_add(1, SeqCst)
            }
            tx.send(());
        })
    }
    // Wait for threads to finish
    for _ in range(0u, NUM_THREADS) { rx.recv(); }
    println!("{}" , counter.load(SeqCst));
}
