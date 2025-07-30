use std::{sync::{mpsc, Arc}, thread, vec};

fn main() {
    let items = vec![
        "sword".to_string(),
        "shield".to_string(),
        "potion".to_string(),
    ];

    // sync_channel() to limit the number of messages in the channel
    // to the number of items, preventing overflow.
    let (sender, receiver) = mpsc::sync_channel::<String>(items.len());

    let sender_arc = Arc::new(sender);

    for item in items.clone().into_iter() {
        thread::spawn({
            let thread_sender = Arc::clone(&sender_arc);
            let item = item.clone();
            move || {
                thread_sender.send(format!("Worker {}: Task completed!", item)).unwrap();
            }
        });
    }

    for _ in 0..items.len() {
        let item = receiver.recv().unwrap();
        println!("Crabby received: {}", item);
    }

    // channel() would allow unlimited messages, which could lead to a deadlock
    // if the main thread waits for all messages to be received.
    let (sender, receiver) = mpsc::channel::<String>();

    let sender_arc = Arc::new(sender);

    for item in items.clone().into_iter() {
        thread::spawn({
            let thread_sender = Arc::clone(&sender_arc);
            let item = item.clone();
            move || {
                thread_sender.send(format!("Worker {}: Task completed!", item)).unwrap();
            }
        });
    }
    drop(sender_arc); // Close the sender to prevent deadlock

    while let Ok(item) = receiver.recv() {
        println!("Crabby received: {}", item);
    }

    println!("===============================================================");
    challenge();

}

fn challenge() {
    let loots =vec![10, 20, 30, 40, 50];
    let mut crabby_gold_coin = 100;

    let (sender, receiver) = mpsc::sync_channel::<i32>(5);
    let sender_arc = Arc::new(sender);

    for loot in loots.clone().into_iter() {
        thread::spawn({
            let thread_sender = Arc::clone(&sender_arc);

            move || {
                thread_sender.send(loot).unwrap();
            }
        });
    }

    for _ in 0..loots.len() {
        let loot = receiver.recv().unwrap();
        crabby_gold_coin += loot;
        println!("Crabby received loot: {}", loot);
    }

    println!("Crabby's total gold coins: {}", crabby_gold_coin);
}
