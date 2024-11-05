use std::pin::Pin;

use pin_project::pin_project;

#[derive(Default)]
struct BagOfApples {
    _require_pin: std::marker::PhantomPinned,
}

impl BagOfApples {
    fn sell_one(self: Pin<&mut Self>) {
        println!("Sold an apple!");
    }
}

#[derive(Default)]
struct BagOfOranges {
    _require_pin: std::marker::PhantomPinned,
}

impl BagOfOranges {
    fn sell_one(self: Pin<&mut Self>) {
        println!("Sold an orange!");
    }
}

#[derive(Default)]
struct BagOfBananas {
    _require_pin: std::marker::PhantomPinned,
}

impl BagOfBananas {
    fn sell_one(self: Pin<&mut Self>) {
        println!("Sold a banana!");
    }
}

// ### Naive example.
// struct FruitStand {
//     apples: BagOfApples,
//     oranges: BagOfOranges,
//     bananas: BagOfBananas,

//     total_sold: usize,
// }

// ### Boxed variant.
// struct FruitStand {
//     apples: Pin<Box<BagOfApples>>,
//     oranges: Pin<Box<BagOfOranges>>,
//     bananas: Pin<Box<BagOfBananas>>,

//     total_sold: usize,
// }

// impl FruitStand {
//     fn new() -> Self {
//         FruitStand {
//             apples: Box::pin(BagOfApples::default()),
//             oranges: Box::pin(BagOfOranges::default()),
//             bananas: Box::pin(BagOfBananas::default()),

//             total_sold: 0,
//         }
//     }

//     fn sell_one_of_each(&mut self) {
//         self.apples.as_mut().sell_one();
//         self.oranges.as_mut().sell_one();
//         self.bananas.as_mut().sell_one();
//         self.total_sold += 3;
//     }
// }

// ### Naive pinned fruit stand.
// #[derive(Default)]
// struct FruitStand {
//     apples: BagOfApples,
//     oranges: BagOfOranges,
//     bananas: BagOfBananas,

//     total_sold: usize,
// }

// impl FruitStand {
//     fn sell_one_of_each(self: Pin<&mut Self>) {
//         self.apples.sell_one();
// // error[E0599]: no method named `sell_one` found for struct `BagOfApples` in the current scope
// //   --> examples\05_project.rs:85:21
// //    |
// // 4  | struct BagOfApples {
// //    | ------------------ method `sell_one` not found for this struct
// // ...
// // 85 |         self.apples.sell_one();
// //    |                     ^^^^^^^^ method not found in `BagOfApples`
// //    |
// // help: consider pinning the expression
// //    |
// // 85 ~         let mut pinned = std::pin::pin!(self.apples);
// // 86 ~         pinned.as_mut().sell_one();
// //    |

//         self.oranges.sell_one();
//         self.bananas.sell_one();
//         self.total_sold += 3;
//     }
// }

// ### Projected variant.
#[derive(Default)]
#[pin_project]
struct FruitStand {
    #[pin]
    apples: BagOfApples,
    #[pin]
    oranges: BagOfOranges,
    #[pin]
    bananas: BagOfBananas,

    total_sold: usize,
}

impl FruitStand {
    fn sell_one_of_each(mut self: Pin<&mut Self>) {
        let self_projected = self.as_mut().project();

        self_projected.apples.sell_one();
        self_projected.oranges.sell_one();
        self_projected.bananas.sell_one();
        *self_projected.total_sold += 3;
    }
}

fn main() {}
