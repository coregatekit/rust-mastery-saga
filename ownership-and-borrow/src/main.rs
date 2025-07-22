fn main() {
    let crabby_treasure = String::from("gold coins");
    // let crabby_treasure_copy = crabby_treasure; // take ownership from crabby_treasure
    let crabby_treasure_copy = crabby_treasure.clone(); // copy value from crabby_treasure
    println!("Crabby treasure: {}", crabby_treasure);
    println!("Crabby treasure copy: {}", crabby_treasure_copy);

    // Rule of borrowing
    // 1. Borrow value by referencing it '&'.
    // 2. Owner can lend immutable and mutable
    // - Immutable reference: Owner allows others to look at this value but not change it
    // - Mutable reference: Owner can let someone borrow and change value. But only one person can borrow it at a time

    // Immutable
    let crabby_treasure = String::from("gold coins");
    let borrow_treasure = &crabby_treasure;

    println!("Borrowed treasure: {}", borrow_treasure);
    println!("Original treasure: {}", crabby_treasure);

    // Mutable
    let mut crabby_treasure = String::from("gold coins");

    let borrow_treasure = &mut crabby_treasure;
    // let borrow_treasure_2 = &mut crabby_treasure; // error cannot borrow more than one at that time
    borrow_treasure.push_str(" and silver coins"); // change some value

    println!("Borrowed treasure: {}", borrow_treasure); // show the value
    println!("Original treasure: {}", crabby_treasure); // show the value also change from borrower's change
    
    example();
}

fn example() {
    let mut treasure = String::from("gold coins");
    
    // Multiple friends borrow immutably!
    let friend1 = &treasure;
    let friend2 = &treasure;
    println!("Friend 1 sees: {}", friend1);
    println!("Friend 2 sees: {}", friend2);
    
    // Trusted friend borrows mutably
    let trusted_friend = &mut treasure;
    trusted_friend.push_str(" and silver coins");
    
    println!("Trusted friend updates: {}", trusted_friend);
}
