use std::time::Instant;

// Begin of time
pub fn start() -> Instant {
    Instant::now()
}

// Calculate the time elapsed since the start function and print this time
pub fn end(i: Instant) {
    let duration = i.elapsed();
    println!("Time elapsed: {:?}", duration);
}