use std::sync::atomic::{AtomicUint, Relaxed, INIT_ATOMIC_UINT};

const NUM_THREADS: uint = 20;
const NUM_INCREMENTS: u32 = 1000000;

static COUNTER: AtomicUint = INIT_ATOMIC_UINT;

fn main() {
    let (tx, rx) = channel();
    for _ in range(0u, NUM_THREADS) {
        let tx = tx.clone();
        spawn(proc() {
            for _ in range(0u32, NUM_INCREMENTS) {
                COUNTER.fetch_add(1, Relaxed);
            }
            tx.send(());
        })
    }
    // Wait for threads to finish
    for _ in range(0u, NUM_THREADS) { rx.recv(); }
    println!("{}" , COUNTER.load(Relaxed));
}
