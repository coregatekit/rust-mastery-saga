fn main() {
    let crabby_energy = 55;
    
    if crabby_energy > 75 {
        println!("Crabby is full of energy and ready to adventure");
    } else if crabby_energy > 50 {
        println!("Crabby is a bit tired but can keep going.");
    } else {
        println!("Crabby needs to rest...");
    }
    
    let enemy = "slime";
    
    match enemy {
        "goblin" => println!("Crabby attacks with claws!"),
        "slime" => println!("Crabby uses a water blast!"),
        "dragon" => println!("Crabby runs for cover!"),
        _ => println!("Crabby doesn't know what to do..."),
    }
    
    let mut apples = 0;

    loop {
        apples += 1;
        println!("Crabby collects an apple. Total: {}", apples);
        
        if apples == 5 {
            break;
        }
    }
    
    let weather = "rainy";
    
    if weather == "sunny" {
        println!("Crabby will cross the river by swimming!");
    } else if weather == "rainy" {
        println!("Crabby will build a bridge to stay dry!");
    } else {
        println!("Crabby will wait for better weather.");
    }
    
    let enemy = "troll";
    
    match enemy {
        "goblin" => println!("Crabby uses his rusty sword to attack!"),
        "troll" => println!("Crabby sets a trap!"),
        "dragon" => println!("Crabby runs for cover!"),
        _ => println!("Crabby is confused.."),
    }
    
    let mut wood = 0;

    loop {
        wood += 1;
        println!("Crabby gathered {} pieces of wood!", wood);
        
        if wood == 10 {
            println!("Crabby finished the boat!");
            break;
        }
    }
}
