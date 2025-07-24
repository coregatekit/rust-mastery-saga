
// Box<T>: Crabby's Trusty Storage for Recursion
// This example demonstrates how to use Box<T> to store recursive data structures in Rust.

// struct Crab<'a> {
//     name: &'a str,
//     member: Option<Crab<'a>>,
// }
// Crab {
//     name: "Crab1",
//     member: Some(Crab {
//         name: "Crab2",
//         member: Some(Crab {
//             name: "Crab3",
//             member: None,
//         }),
//     }),}),
// }
// Recursive storing of Crab instances using Box<T>

// struct Crab<'a> {
//     name: &'a str,
//     member: Option<Box<Crab<'a>>>, // Using Box<T> to allow recursive types
// }
// Crab {
//     name: "Crab1",
//     member: Some(Box::new(Crab {
//         name: "Crab2",
//         member: Some(Box::new(Crab {
//             name: "Crab3",
//             member: None,
//         })),
// })),
// }

// Rc<T>: Crabby's Mirror Image Spell (Reference Counting)
// Rc<T> allows multiple ownership of data, enabling shared access to the same data without needing to clone it.

use std::cell::RefCell;
use std::rc::Rc;

// RefCell<T>: Crabby's Flexible Container
// RefCell<T> allows for mutable access to data even when the data is shared, enabling interior mutability.

fn main() {
    // let sword = String::from("Sword");
    // //
    // // let loot_1 = sword.clone();
    // // let loot_2 = sword.clone(); // Cause of memory performance issues
    //
    // let epic_loot = Rc::new(sword);
    //
    // let loot_1 = Rc::clone(&epic_loot);
    // let loot_2 = Rc::clone(&epic_loot); // Cloning Rc<T> is cheap, just increments the reference count
    // // loot 1 and loot 2 now share ownership of the same String data
    //
    // let gold = Box::new(10);
    // // let epic_loot = Rc::new(gold); // -> immutable
    // let epic_loot = Rc::new(RefCell::new(gold)); // Using RefCell<T> to make it mutable

    println!("==================================================");

    let gold = Box::new(10);
    let epic_loot = Rc::new(RefCell::new(gold));

    let loot_1 = Rc::clone(&epic_loot);
    let loot_2 = Rc::clone(&epic_loot);

    **loot_1.borrow_mut() += 20;
    **loot_2.borrow_mut() += 100;

    println!("Epic Loot: {}", epic_loot.borrow());

    // Recap
    // Box<T>: Allocates a value on the heap in safe mode.
    // Rc<T>: Enables multiple pointers to share ownership.
    // RefCell<T>: Allows interior mutability, enabling mutable access to shared data.
    challenge();
}

fn challenge() {
    let chest = 10; // Use Box<T> to prevent memory leaks
    // let chest = Box::new(10); // Box<T> to allocate on the heap

    let shared_chest = Rc::new(RefCell::new(chest)); // shared ownership with interior mutability

    *shared_chest.borrow_mut() += 10;
    *shared_chest.borrow_mut() += 5;
    // **shared_chest.borrow_mut() += 10;
    // **shared_chest.borrow_mut() += 5;

    println!("Gold: {}", shared_chest.borrow());
}
