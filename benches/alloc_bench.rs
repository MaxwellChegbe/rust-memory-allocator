use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_memory_allocator::BumpAllocator; 

fn criterion_benchmark(c: &mut Criterion) {
    let mut arena = BumpAllocator::new(1024);

    c.bench_function("bump_alloc_u64", |b| {
        b.iter(|| {
            // black_box prevents the compiler from "optimizing away" the code
            arena.allocate(black_box(100u64));
            arena.reset(); 
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);