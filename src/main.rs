use std ::mem;

// A simple bump allocator
//it owns a fixed buffer and a pointer
//that always points to the next free byte

struct BumpAllocator {
    buffer: Vec<u8>, //raw memory we mange
    next: usize,     //pointer to the next free byte
}

// this function will handle the type for our memory allocation
//addr means address of where the pointer is at currently
// align is the boundary that can accomaodate our typed value
// the function will return the accurate aligned location 

fn align_up (addr: usize, align: usize ) -> usize{
   // while addr % align != 0 {
   //     addr += 1;
    //}
    //addr

    // (---using Bitwise math logic-----)
    (addr + align - 1) & !(align-1)
}

impl BumpAllocator {
    // creating a fixed sized buffer
    fn new(size: usize) -> Self {
        BumpAllocator{
            buffer: vec![0; size], // allocating sized zero byte
            next: 0,
        }
    }

    // alloacting a value of type T inside the buffer
    // and return a mutable refernce to it
    
    fn allocate <'a, T>(&'a mut self, value:T)-> Option<&'a mut T> {
        // Required allignmenmt for type T (e.g., u32->4, u64 -> 8)
        let align = mem::align_of::<T>();

        //size of byte  of type T
        let size = mem::size_of::<T>();

        //move pointer 'next' foward until it satisfies alignment

        let aligned = align_up(self.next, align);

        //handle buffer size
        if aligned +  size > self.buffer.len(){
            return None;
        }

        // Get a raw pointer to the buffer's memory
        let ptr= unsafe {
            //as_mut_ptr gives *mut u8
            //add(aligned) moves foward "aligned" bytes
            //cast to *mut T so we can write a t there
            self.buffer.as_mut_ptr().add(aligned) as *mut T
        };
        //Write the value into memory without reading or dropping any data
        unsafe {
            ptr.write(value);
        }

        //advancing the pointer past this allocation
        self.next= aligned + size;

        //convert raw pointer into a mutable ref
        //This is safe because: memory is valid, alignment is correct and no aliasing
        unsafe{
            Some(&mut *ptr)
        }
    }

    //reseting the allocator. freeing evrything at once
    fn reset(&mut self){
        self.next=0;
    }
}

fn main(){
    let mut sallocator = BumpAllocator::new(64);
    {
    let x = sallocator.allocate(42u32).expect("allocation failed");
        println!("x={}", x);
    }

    let y = sallocator.allocate(3.14f64).expect("allocation failed");
    println!("y={}", y);

    sallocator.reset();

    let z = sallocator.allocate(100u16).unwrap();
    println!("z{}", z);
}
