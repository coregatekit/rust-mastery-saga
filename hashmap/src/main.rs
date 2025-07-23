use std::collections::HashMap;

fn main() {
    let mut treasures = HashMap::new();

    treasures.insert("Gold coins", 10);
    treasures.insert("Gem", 1000);

    if let Some(gem) = treasures.get("Gem") {
        println!("Gem: {}", gem);
    }
    // update value
    if let Some(gem) = treasures.get_mut("Gem") {
        *gem += 10; // pointer to address of gem to modify value
        println!("Gem: {}", gem);
    }

    // convert to tuple (key, value) for access value
    for (treasure, count) in treasures.iter() {
        println!("{} treasure: {}", treasure, count);
    }

    let ruby_count = treasures.get("Ruby").unwrap_or(&0);
    // handle missing key if no key it will return default value in this case is zero
    println!("Ruby count: {}", ruby_count);

    // to remove item from hashmap
    println!("treasures: {:?}", treasures);
    treasures.remove("Gem");
    println!("treasures: {:?}", treasures);
    
    println!("=======================================================================");
    challenge();
}

fn challenge() {
    let mut treasures: HashMap<&str, i32> = HashMap::new();
    
    treasures.insert("Silver coins", 100);
    treasures.insert("Ruby", 3);
    
    println!("Treasures: {:?}", treasures);
    
    if let Some(ruby) = treasures.get_mut("Ruby") {
        *ruby += 5;
    }
    println!("Treasures: {:?}", treasures);
}
