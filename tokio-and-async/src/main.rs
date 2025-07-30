use std::sync::Arc;

use tokio::{self, sync::mpsc::{self, Sender}};

async fn gather_herbs() {
    println!("Crabby is gathering herbs!");
}

async fn collect_gold_coins() {
    println!("Crabby is collecting gold coins!");
}

async fn send_item<'a>(
    item: &'a str,
    tx: Arc<Sender<&'a str>>,
) {
    tx.send(item).await.unwrap();
}

#[tokio::main]
async fn main() {
    // tokio::spawn(async {
    //     println!("Crabby is gathering herbs!");
    // })
    // .await
    // .unwrap();

    // tokio::spawn(async {
    //     println!("Crabby is collecting gold coins!");
    // })
    // .await
    // .unwrap();

    let crafting_task = tokio::spawn(async {
        println!("Crabby is crafting a potion!");
    });

    let learning_task = tokio::spawn(async {
        println!("Crabby is learning a new skill!");
    });

    let herbs_task = tokio::spawn(gather_herbs());
    let coins_task = tokio::spawn(collect_gold_coins());

    // Wait for both tasks to complete
    let _ = tokio::join!(crafting_task, learning_task, herbs_task, coins_task);


    let items = vec!["herbs", "gold coins", "gems"];

    let (tx, mut rx) = mpsc::channel::<&str>(items.len());
    let tx = Arc::new(tx);

    for item in items.clone().into_iter() {
        tokio::spawn({
            let cloned_tx = Arc::clone(&tx);
            async move {
                send_item(item, cloned_tx).await;
            }
        });
    }
    drop(tx);

    for _ in 0..items.len() {
        let result = rx.recv().await.unwrap();
        println!("Crabby received: {}", result);
    }

    println!("===================================================================");
    challenge().await;
}


async fn ch_gather_herbs() {
    tokio::time::sleep(tokio::time::Duration::from_millis(
        rand::random::<u64>() % 901 + 100
    )).await;
    println!("Crabby is gathering herbs...");
}

async fn ch_collect_gold_coins() {
    tokio::time::sleep(tokio::time::Duration::from_millis(
        rand::random::<u64>() % 901 + 100
    )).await;
    println!("Crabby is collecting gold coins...");
}

async fn ch_fight_monster() {
    tokio::time::sleep(tokio::time::Duration::from_millis(
        rand::random::<u64>() % 901 + 100
    )).await;
    println!("Crabby is fighting a monster!");
}

async fn challenge() {
    let task_1 = tokio::spawn(ch_gather_herbs());
    let task_2 = tokio::spawn(ch_collect_gold_coins());
    let task_3 = tokio::spawn(ch_fight_monster());

    let _ = tokio::join!(task_1, task_2, task_3);
}

