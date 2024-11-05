use std::pin::Pin;

pub trait BagOfFruit {
    fn set_capacity(self: Pin<&mut Self>, capacity: usize);
}

pub struct BagOfBananas {
    capacity: usize,
    _require_pin: std::marker::PhantomPinned,
}

impl BagOfFruit for BagOfBananas {
    fn set_capacity(mut self: Pin<&mut Self>, capacity: usize) {
        // SAFETY: BagOfBananas requires pinning. We have reviewed the code
        // in this function  to ensure that we do not move the BagOfBananas
        // instance via the reference we obtain from here.
        let self_mut: &mut BagOfBananas = unsafe { self.as_mut().get_unchecked_mut() };

        self_mut.capacity = capacity;
    }
}

pub struct BagOfOranges {
    capacity: usize,
}

impl BagOfFruit for BagOfOranges {
    fn set_capacity(mut self: Pin<&mut Self>, capacity: usize) {
        // This type does not require pinning, so we can simply dereference.
        let self_mut: &mut BagOfOranges = &mut self;

        self_mut.capacity = capacity;
    }
}

pub fn example_pinned_integer() {
    let pinned = Box::pin(42);
    println!("Pinned integer: {}", *pinned);
}

fn main() {
    example_pinned_integer();
}
