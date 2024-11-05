#![allow(clippy::new_without_default)]

use std::ptr;

// BAD CODE: This example is intentionally wrong.
pub struct BagOfApples {
    count: usize,

    // In the example code, this self-reference is mostly useless.
    // This is just to keep the example code simple - the emphasis is
    // on the effects of pinning, not why a type may be designed to need it.
    self_reference: *mut BagOfApples,
}

impl BagOfApples {
    pub fn new() -> Self {
        BagOfApples {
            count: 0,
            // We cannot set this here because we have not yet
            // created the BagOfApples - there is nothing to reference.
            self_reference: ptr::null_mut(),
        }
    }

    /// Call this after creating a BagOfApples to initialize the instance.
    pub fn initialize(&mut self) {
        self.self_reference = self;
    }

    pub fn count(&self) -> usize {
        assert!(
            !self.self_reference.is_null(),
            "BagOfApples is not initialized"
        );

        // SAFETY: Simple read-only access to the count field, which
        // is safe. We do it via the pointer for example purposes.
        unsafe { (*self.self_reference).count }
    }
}
// BAD CODE: This example is intentionally wrong.

pub fn example_without_move() {
    let mut bag = BagOfApples::new();
    bag.initialize();
    println!("Apple count: {}", bag.count());
}

pub fn example_with_move() {
    // BAD CODE: This example is intentionally wrong.
    let mut bag = BagOfApples::new();
    bag.initialize();

    // We move the bag into a box, which is a different memory location.
    // Invalid code: it is not legal to move the bag after creation because it is
    // self-referential. BagOfApples is an unsound type because it allows this.
    let boxed_bag = Box::new(bag);

    // This will result in invalid memory access, causing undefined behavior.
    // It may appear to work if you get lucky but this does not make it okay.
    println!("Apple count: {}", boxed_bag.count());
    // BAD CODE: This example is intentionally wrong.
}

fn main() {
    example_with_move();
    example_without_move();
}
