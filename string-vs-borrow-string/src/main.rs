// String and &str(borrow string)

// A String in Rust is an owned, growable string type stored on heap.
// Since it's heap-allocated, it can store data that we can modify.
// String can re-allocate address size

// A &str is a borrowed string slice.
// It's an immutable reference to a string data that lives elsewhere.
// We can borrow a string without taking ownership. So we can readonly, unable to modify it
// &str allocate address in memory at runtime

fn main() {
    // Create and Modifying
    let mut crabby_string = String::from("Crabby's treasure map");
    crabby_string.push_str(" - adding a mysterious island");
    println!("{}", crabby_string);
    
    // Borrowing a slice &str
    let treasure_map = String::from("Hidden treasure in the Crab Cave");
    
    let borrowed_part: &str = &treasure_map[0..6];
    println!("Crabby borrowed: {}", borrowed_part);
    
    // Converting between String and &str
    let crabby_string = String::from("silver coins");
    
    // Borrow a string slice from a String
    let borrowed_slice: &str = &crabby_string;
    let borrowed_slice2: &str = crabby_string.as_str(); // another way to borrow
    _ = borrowed_slice2;
    
    // Convert &str back to String 
    let converted_string = borrowed_slice.to_string(); // this behavior did not take ownership it will re-allocate a new String
    
    println!("Borrowed string: {}", borrowed_slice);
    println!("Converted back to String: {}", converted_string);
    
    challenge();
}

fn challenge() {
    let map = String::from("Old map");
    
    let borrowed_map = map.as_str();
    
    let mut crabby_map = borrowed_map.to_string();
    crabby_map.push_str(" to new map");
    println!("Challenge map: {}", crabby_map);
}
