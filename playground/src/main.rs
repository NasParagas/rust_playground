use std::time::Instant;
use quanta::Clock;

fn main() {
    let clock = Clock::new();
    const N:u32 = 1_000_000;

    let istart = Instant::now();
    let mut istop = istart;
    for _ in 1..N {
        istop = Instant::now();
    }
    println!("std::time:Instant::now() overhead = {:?}",
        istop.duration_since(istart));

    let start = clock.now();
    let mut stop = start;
    for _ in 1..N {
        stop = clock.now();
    }
    println!("quanta::clock::now() overhead = {:?}",
        stop.duration_since(start));
}

