use std::ptr;

/**
This example is about why pinning is needed.
**/

struct SelfReferential {
    data: String,
    /// raw pointer to a string. Pointer has not safety guarantee.
    /// The pointer does not update if the data being pointed to moves.
    self_pointer: *const String,
}

impl SelfReferential {
    fn new(data: String) -> SelfReferential {
        let mut sr = SelfReferential {
            data,
            self_pointer: ptr::null(),
        };
        sr.self_pointer = &sr.data as *const String;
        sr
    }

    fn print(&self) {
        unsafe {
            // can only deref a raw pointer in unsafe block
            println!("self_pointer: {}", *self.self_pointer);
        }
    }
}

fn main() {
    // expose the danger of moving the struct by creating two instances of the
    // SelfReferential struct, swap these instances in memory.
    // If you try to run the code, you will get an error, which is likely a segmentation fault. A
    // segmentation fault is an error caused by accessing memory that does not belong to the
    // program. We can see that moving a struct with a reference to itself can be dangerous.
    // Pinning ensures that the future remains at a fixed memory address. This is important
    // because futures can be paused or resumed, which can change the memory address.
    let first = SelfReferential::new("first".to_string());
    let moved_first = first; // Move the struct
    moved_first.print();
}
