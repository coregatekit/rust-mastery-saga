// trait Weapon {
//     fn attack(&self);
// }

// struct Sword;
// struct Bow;

// impl Weapon for Sword {
//     fn attack(&self) {
//         println!("Swinging the sword!");
//     }
// }

// impl Weapon for Bow {
//     fn attack(&self) {
//         println!("Shooting an arrow!");
//     }
// }
// Using Traits as Type
// Traits can be used as types in Rust, allowing for polymorphism.
// Polymorphism is the ability to use a single interface to represent different underlying data types.
// This is useful for writing generic code that can work with different types that implement the same trait
// Example: Using a trait as a type in a function parameter

// Dynamic Dispatch (dyn Trait)

// fn get_weapon(weapon_type: &str) -> Weapon { // Error because Rust doesn't know the size of the trait
// fn get_weapon(weapon_type: &str) -> Box<dyn Weapon> { // Use Box to enable dynamic dispatch
//     // dyn: It's means dynamics trait that to tell Rust to calculate the size of the trait at runtime
//     match weapon_type {
//         "sword" => Box::new(Sword),
//         "bow" => Box::new(Bow),
//         _ => panic!("Unknown weapon type!"),
//     }
// }

// fn weapon_attack(weapon: Box<dyn Weapon>) { // Use dyn as a type
//     weapon.attack();
// }
// fn weapon_attack(weapon: &dyn Weapon) { // or use as borrowed reference but be careful with lifetimes
//     weapon.attack();
// }


// Static Dispatch (Trait Bounds) impl Trait

// fn get_weapon_static(weapon_type: &str) -> impl Weapon {
    // Error: It's can return as a Sword or Bow. But Rust can't estimate the size of the return type
    // match weapon_type {
    //     "sword" => Sword,
    //     "bow" => Bow,
    //     _ => panic!("Unknown weapon type!"),
    // }
// }

// Rust's compiler will generate function for Sword that implement the Weapon trait as 1:1.
// Therefore, it can't make this function return both Sword and Bow at the same time.

// fn get_sword() -> impl Weapon {
//     // Generate get_sword to return Sword at compile time
//     Sword
// }
// fn get_bow() -> impl Weapon {
//     // Generate get_bow to return Bow at compile time
//     Bow
// }

// fn weapon_attack(weapon: impl Weapon) {
//     weapon.attack();
// }

// trait Weapon {
//     fn attack(&self);
// }

// struct Sword;
// struct Bow;

// impl Weapon for Sword {
//     fn attack(&self) {
//         println!("Swinging the sword!");
//     }
// }

// impl Weapon for Bow {
//     fn attack(&self) {
//         println!("Shooting an arrow!");
//     }
// }

// Rust's compiler will generate a functions for all types that implement the Weapon trait.
// fn weapon_attack<T: Weapon>(weapon: T) {
//     weapon.attack();
// }
// fn weapon_attack(weapon: Sword) {
//     weapon.attack();
// }
// fn weapon_attack(weapon: Bow) {
//     weapon.attack();
// }

fn main() {
    println!("Hello, world!");
    challenge();
}

trait Gear {
    fn use_gear(&self);
}

struct Sword;
struct Bow;
struct Potion;

impl Gear for Sword {
    fn use_gear(&self) {
        println!("Swinging the sword!");
    }
}

impl Gear for Bow {
    fn use_gear(&self) {
        println!("Shooting an arrow!");
    }
}

impl Gear for Potion {
    fn use_gear(&self) {
        println!("Drinking the potion!");
    }
}

fn use_gear<T: Gear>(item: T) {
    item.use_gear();
}

fn challenge() {
    let crabby_sword = Sword;
    let crabby_bow = Bow;
    let crabby_potion = Potion;    

    use_gear(crabby_sword);
    use_gear(crabby_bow);
    use_gear(crabby_potion);
}

