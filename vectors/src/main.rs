// Vector is like a flexible array, owned and heap allocated
// Note:
// .iter() is borrowing
// .into_iter() is moving ownership

fn main() {
    let mut treasures: Vec<String> = Vec::new();

    treasures.push(String::from("Gold coins"));
    treasures.push("Emerald".to_string());
    println!("Crabby's treasure {:?}", treasures);

    let treasures = vec!["Gold coins", "Gem", "Sword"];
    let favorite_treasure = &treasures[1];
    println!("Favorite treasure {:?}", favorite_treasure);

    for treasure in treasures.iter() {
        println!("Crabby collects {}", treasure);
    }

    let mut treasures = vec!["Gold coins", "Gem", "Sword"];
    println!("treasure {:?}", treasures);
    let last_treasure = treasures.pop();
    println!("treasure {:?}", treasures);
    println!("Last treasure {:?}", last_treasure);
    let second_treasure = treasures.remove(0);
    println!("treasure {:?}", treasures);
    println!("Second treasure {:?}", second_treasure);

    println!("=========================================");
    let mut treasures = vec!["Gold coins", "Gem", "Sword"];
    println!("treasure {:?}", treasures);

    let treasure_length = treasures.len();
    let treasure_capacity = treasures.capacity();
    println!("Treasure length {:?}", treasure_length);
    println!("Treasure capacity {:?}", treasure_capacity);
    let last_treasure = treasures.pop();
    println!("last_treasure {:?}", last_treasure);

    println!("treasure {:?}", treasures);
    println!("Treasure length {:?}", treasure_length);
    println!("Treasure capacity {:?}", treasure_capacity);

    println!("=========================================");
    challenge();
}

fn challenge() {
    let mut items = vec!["Gold", "Silver", "Ruby Gem"];

    items.remove(1);
    // items.push("Diamonds");

    println!("Items: {:?}", items);
    println!("Item length: {:?}", items.len());
    println!("Item capacity: {:?}", items.capacity());
}
