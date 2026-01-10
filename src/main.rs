use rust_memory_allocator::BumpAllocator;

fn main() {
    let mut arena = BumpAllocator::new(1024);

    // demo of allocating different types
    if let Some(num) = arena.allocate(42u32) {
        print!("Allocated u32: {}", num);

    }

    if let Some(flag) = arena.allocate(true) {
        println!("Allocated bool:{}", flag);

    }

    arena.reset(); //cleaning up bump allocator
}