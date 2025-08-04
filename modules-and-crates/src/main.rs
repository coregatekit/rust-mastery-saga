// note: :: is equal to /
// and - (hyphen) is equal to _ (underscore)

use modules_and_crates::calculator;
use modules_and_crates::challenge;
use modules_and_crates::challenge::weapons;
use modules_and_crates::challenge::maps::use_item as use_map_item;

// ========================== Challenge ==========================
// Create module for potions
// Create module for weapons
// Create module for maps
// ========================== Challenge ==========================

fn main() {
    let a = 10;
    let b = 20;

    let result = calculator::add::add(a, b);
    println!("The sum of {} and {} is {}", a, b, result);

    println!("==========================");
    challenge::potions::use_item();
    weapons::use_item();
    use_map_item();
    println!("==========================");
}

