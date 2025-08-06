// Single Responsibility Principle (SRP) violation example
// This code demonstrates a violation of the Single Responsibility Principle (SRP).
// A class should have only one reason to change, but here the Character struct
// is responsible for both character attributes and quest management, which can lead to
// complex and error-prone code.

use std::sync::Arc;

struct Character {
    name: String,
    health: u32,
    current_quest: String,
}

// Character also handles quests, makeing changes more complex and error-prone.
// quest should not be part of Character's responsibility.
impl Character {
    fn take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);
    }

    fn start_quest(&mut self, quest_name: &str) {
        self.current_quest = quest_name.to_string();
    }
}

// Open/Closed Principle (OCP) violation example
// This code demonstrates a violation of the Open/Closed Principle (OCP).
// The Character struct is not open for extension, as adding new features requires modifying existing code.

struct Mage;

impl Mage {
    fn use_ability(&self, aibility_type: &str) {
        match aibility_type {
            "fireball" => println!("Casting Fireball!"),
            "teleport" => println!("Teleporting!"),
            _ => println!("Unknown ability!"), // <- modifying use_ability to add new abilities, which risks breaking existing code.
        }
    }
}

// Liskov Substitution Principle (LSP) violation example
// This code demonstrates a violation of the Liskov Substitution Principle (LSP).

trait Mount {
    fn ride(&self);
}

struct Rock;

impl Mount for Rock {
    fn ride(&self) {
        // Rock doesn't fit the Mount behavior, breaking substitution. Rock cannot be ridden. So we should not implement Mount for it.
        panic!("Rocks can't be ridden!");
    }
}

// Interface Segregation Principle (ISP) violation example
// This code demonstrates a violation of the Interface Segregation Principle (ISP).
// The Character struct implements a large interface that includes methods not relevant to all characters.

trait Character2 {
    fn attack(&self);
    fn heal(&self);
}

struct Archer;

impl Character2 for Archer {
    fn attack(&self) {
        println!("Archer attacks with a bow!");
    }

    fn heal(&self) {
        // Character forces Archer to implement unused methods. It's not relevant for Archer to heal.
    }
}

// Dependency Inversion Principle (DIP) violation example
// This code demonstrates a violation of the Dependency Inversion Principle (DIP).

struct Sword;

impl Sword {
    fn use_weapon(&self) {
        println!("Swinging the sword!");
    }
}

struct Warrior {
    weapon: Sword,
}

impl Warrior {
    fn fight(&self) {
        self.weapon.use_weapon(); // Warrior is tightly coupled to Sword, making it hard to change weapon types.
    }
}
