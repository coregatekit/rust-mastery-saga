use std::sync::mpsc::channel;

fn main() {
    let mut treasure_collected = 0;
    loop {
        treasure_collected += 1;
        println!("Crabby has collected {} treasures!", treasure_collected);
        
        if treasure_collected >= 5 {
            break;
        }
    }
    
    let mut energy = 10;
    while energy > 0 {
        println!("Crabby gathers treasures. Engery left: {}", energy);
        energy -= 1;
    }
    println!("Crabby is too tired to continue digging!");
    
    let treasures = ["Gold coins", "Silver coins", "Ruby", "Emerald"];
    for treasure in treasures.iter() {
        println!("Crabby collects: {}", treasure);
    }

    let treasures = ["Gold", "Silver", "Poison", "Ruby"];
    for treasure in treasures.iter() {
        if treasure == &"Poison" {
            println!("Crabby avoids the Poison!");
            continue;
        }
        println!("Crabby collects: {}", treasure);
    }
    
    challenge();
}

fn challenge() {
    let treasures = ["Gold", "Silver", "Crystal", "Emerald", "Jade", "Gold"];
    let mut energy = 5;
    
    for treasure in treasures.iter() {
        if energy == 0 {
            println!("Crabby out of energy!");
            break;
        } else if treasure == &"Ruby Gem" {
            println!("Crabby found Ruby Gem!");
            break;
        }
        
        energy -= 1;
    }
}
