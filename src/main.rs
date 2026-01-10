use std::time::Instant;
use rust_memory_allocator::BumpAllocator; 

fn main() {
    
    let mut arena = BumpAllocator::new(1024 * 1024); // 1MB Arena
    let iterations = 10_000;

    println!("Starting benchmark for {} allocations...", iterations);

    //Benchmark Logic
    let start = Instant::now(); // Start
    for i in 0..iterations {
        // black_box
        let _ = arena.allocate(i as u64); 
    }
    let duration = start.elapsed(); // Stop

    // 3. Results for my siwess defence report
    println!("--- SIWES DEFENCE RESULTS ---");
    println!("Total time: {:?}", duration);
    println!("Average time per allocation: {:?}", duration / iterations);
    
    // my ittle O(1) bump reset 
    let reset_start = Instant::now();
    arena.reset();
    println!("Reset time (Freeing all memory): {:?}", reset_start.elapsed());
}