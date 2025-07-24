// Multi-threading
// Do some multi-threading work here

// Synchronous vs Multi-threading
// Synchronous code runs in a single thread, while multi-threading allows concurrent execution.

use std::{sync::{Arc, Mutex}, thread};

// Atomic Reference Counting (Arc)
// Arc is used to share data safely between threads.

fn main() {
    thread_example();
    arc_example();
    mutex_example();
    challenge();
}

fn thread_example() {
    let threaded = thread::spawn(|| {
        println!("Crabby is cooking a potion!");
    });

    threaded.join().unwrap(); // Wait for the thread to finish

    let treasure_thread = thread::spawn(|| {
        println!("Crabby is finding treasure!");
    });

    let potion_thread = thread::spawn(|| {
        println!("Crabby is brewing a potion!");
    });

    potion_thread.join().unwrap();
    treasure_thread.join().unwrap();
}

fn arc_example() {
    let treasure = Arc::new("Golden Sword!");    

    // These will not work because Rc is not thread-safe
    // let thread_1 = thread::spawn({
    //     let treasure_clone = Rc::clone(&treasure);
    //     move || {
    //         println!("Crabby's clone found: {}", treasure_clone);
    //     }
    // });

    // let thread_2 = thread::spawn({
    //     let treasure_clone = Rc::clone(&treasure);
    //     move || {
    //         println!("Crabby's clone found: {}", treasure_clone);
    //     }
    // });

    // Use Arc instead of Rc for thread safety
    let thread_1 = thread::spawn({
        let treasure_clone = Arc::clone(&treasure);
        move || {
            // this thread is the owner of the above treasure_clone
            println!("Crabby's clone found: {}", treasure_clone);
        }
    });

    let thread_2 = thread::spawn({
        let treasure_clone = Arc::clone(&treasure);
        move || {
            println!("Crabby's clone found: {}", treasure_clone);
        }
    });

    thread_1.join().unwrap();
    thread_2.join().unwrap();

    // move command
    // when we pass the global variable into the thread, we use the `move` keyword to transfer ownership of the variable into the thread's closure.
} 

// Mutex
// A Mutex (Mutual Exclusion) is a synchronization primitive that can be used to protect shared data from being accessed by multiple threads simultaneously.

fn mutex_example() {
    let gold_coin = Arc::new(Mutex::new(100));

    let loot_1 = thread::spawn({
        let gold_coin = Arc::clone(&gold_coin);
        move || {
            println!("Thread 1 is trying to add loot...");
            let mut loot = gold_coin.lock().unwrap();
            *loot += 20;
        }
    });

    let loot_2 = thread::spawn({
        let gold_coin = Arc::clone(&gold_coin);
        move || {
            println!("Thread 2 is trying to add loot...");
            let mut loot = gold_coin.lock().unwrap();
            *loot += 100;
        }
    });

    loot_1.join().unwrap();
    loot_2.join().unwrap();
    
    println!("Total gold coins: {}", *gold_coin.lock().unwrap());
}

fn challenge() {
    let crabby_gold = Arc::new(Mutex::new(10));

    let loot_1 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_gold);

        move || {
            let mut gold = crabby_gold_artifact.lock().unwrap();
            *gold += 100;
        }
    });

    let loot_2 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_gold);

        move || {
            let mut gold = crabby_gold_artifact.lock().unwrap();
            *gold += 200;
        }
    });

    let loot_3 = thread::spawn({
        let crabby_gold_artifact = Arc::clone(&crabby_gold);

        move || {
            let mut gold = crabby_gold_artifact.lock().unwrap();
            *gold += 80;
        }
    });

    loot_1.join().unwrap();
    loot_2.join().unwrap();
    loot_3.join().unwrap();

    println!("Crabby's total gold after the challenge: {}", crabby_gold.lock().unwrap());
}