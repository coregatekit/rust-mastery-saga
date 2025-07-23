// Result, Option types

// Option it will return Some, None
// return Some when found data
// return None when not found data

// Result it will return Ok, Err
// return Ok if no error
// return Err if an error occurs

fn main() -> Result<(), String> {
    let treasures = vec!["Gold", "Ruby", "Emerald"];
    let treasure = treasures.get(3); // .get return type Option

    match treasure {
        Some(t) => println!("Found treasure {}", t), // t is found value
        None => println!("No treasure"),
    }

    // let treasure = match treasures.get(3) {
    //     Some(t) => t,
    //     None => panic!("No treasure"),
    // };

    // if let Some(treasure) = treasures.get(3) {
    //     println!("Found treasure {}", treasure);
    // } else {
    //     println!("No treasure");
    // }

    // let chest_result = open_chest(true);
    // match chest_result {
    //     Ok(chest) => println!("{}", chest),
    //     Err(e) => panic!("Err: {}", e),
    // }

    if let Ok(chest_result) = open_chest(true) {
        println!("Found chest: {}", chest_result);
    } else {
        println!("No chest");
    }

    let treasures = vec!["Gold", "Gem", "Sword"];
    let gem = treasures.get(3).unwrap_or(&"Gold"); // if there's an error, it will return default value ("Gold")
    println!("{}", gem);

    let gem = treasures.get(3).unwrap_or_else(|| &"Gold"); // we can also return with closure
    println!("{}", gem);

    // let gem = treasures.get(3).unwrap(); // in case nothing to return just panic
    // println!("{}", gem);

    challenge();

    // If success it will does Ok if automatically
    let chest_result = open_chest(true)?; // if return an error, it will do Err automatically
    Ok(())
}

fn open_chest(is_locked: bool) -> Result<String, String> {
    if is_locked {
        Err("Treasure chest is locked.".to_string())
    } else {
        Ok("Crabby opens the treasure chest and found gold coins.".to_string())
    }
}

fn challenge() {
    let crabby_chest_result = match crabby_open_chest(true) {
        Some(tresure) => tresure,
        None => "The chest is empty".to_string(),
    };

    // let crabby_chest_result = crabby_open_chest(true).unwrap_or_else(|| "The chest is empty".to_string());
    println!("{}", crabby_chest_result);

    let door_result = match crabby_open_door(false) {
        Ok(safe) => safe,
        Err(mimic) => panic!("{}", mimic),
    };
    println!("{}", door_result);
}

fn crabby_open_chest(is_empty: bool) -> Option<String> {
    if is_empty {
        None
    } else {
        Some("You found a treasure!".to_string())
    }
}

fn crabby_open_door(is_danger: bool) -> Result<String, String> {
    if is_danger {
        Err("You found a monster!".to_string())
    } else {
        Ok("The door is safe to open!".to_string())
    }
}
