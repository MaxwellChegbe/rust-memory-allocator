use std::alloc::Layout;

pub struct BumpAllocator {
    buffer: Vec<u8>, // The actual memory "bucket"
    next: usize,     // Offset to the next available byte
}

impl BumpAllocator {
    /// Creates a new allocator with a fixed capacity.
    pub fn new(size: usize) -> Self {
        BumpAllocator {
            buffer: vec![0; size], // Pre-allocates memory on the heap
            next: 0,
        }
    }

    /// Allocates a value of type T into the buffer.
    /// Uses 'a (lifetime) to ensure the reference is valid as long as the allocator exists.
    pub fn allocate<'a, T>(&'a mut self, value: T) -> Option<&'a mut T> {
        let layout = Layout::new::<T>(); // Gets size and alignment for type T
        let align = layout.align();
        let size = layout.size();

        // BITWISE ALIGNMENT LOGIC:
        // This moves the 'next' pointer forward to the nearest multiple of 'align'
        let aligned_pos = (self.next + align - 1) & !(align - 1);

        // BOUNDS CHECKING: 
        // Prevents "Buffer Overflow" by checking if the new data fits
        if aligned_pos + size > self.buffer.len() {
            return None;
        }

        // POINTER ARITHMETIC (UNSAFE):
        // We calculate the exact memory address and cast it to a pointer of type T
        let ptr = unsafe {
            let p = self.buffer.as_mut_ptr().add(aligned_pos) as *mut T;
            p.write(value); // Writes the value into the buffer
            p
        };

        self.next = aligned_pos + size; // Move the 'bump' pointer forward

        // Convert the raw pointer back into a safe Rust reference
        unsafe { Some(&mut *ptr) }
    }

    /// Resets the pointer to zero. This is O(1) "deallocation"
    pub fn reset(&mut self) {
        self.next = 0;
    }
}