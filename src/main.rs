use std::time::Instant;
// Replace 'your_project_name' with your actual name from Cargo.toml
use rust_memory_allocator::BumpAllocator; 

fn main() {
    // 1. Setup
    let mut arena = BumpAllocator::new(1024 * 1024); // 1MB Arena
    let iterations = 10_000;

    println!("Starting benchmark for {} allocations...", iterations);

    // 2. The Benchmark Logic
    let start = Instant::now(); // Start the "Stopwatch"
    for i in 0..iterations {
        // black_box-like behavior: ensure the compiler doesn't skip this
        let _ = arena.allocate(i as u64); 
    }
    let duration = start.elapsed(); // Stop the "Stopwatch"

    // 3. Results for your Report
    println!("--- SIWES DEFENCE RESULTS ---");
    println!("Total time: {:?}", duration);
    println!("Average time per allocation: {:?}", duration / iterations);
    
    // Pro Tip: Mention this O(1) reset in your defense!
    let reset_start = Instant::now();
    arena.reset();
    println!("Reset time (Freeing all memory): {:?}", reset_start.elapsed());
}