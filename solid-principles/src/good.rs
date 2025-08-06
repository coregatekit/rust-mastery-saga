// Single Responsibility Principle (SRP) Example
// This code demonstrates a proper implementation of the Single Responsibility Principle (SRP).
// Each struct has a single responsibility: Character for attributes and Quest for quest management.

struct Character {
    name: String,
    health: u32,
}

impl Character {
    fn take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);
    }
}

struct Quest {
    name: String,
    reward: u32,
}

impl Quest {
    fn start_quest(&self) {
        println!("Starting quest: {}", self.name);
    }
}

// Open/Closed Principle (OCP) Example
// This code demonstrates a proper implementation of the Open/Closed Principle (OCP).
// The Mage struct is designed to be extensible without modifying existing code.

trait Ability {
    fn use_ability(&self);
}

struct Fireball;
struct Heal;

impl Ability for Fireball {
    fn use_ability(&self) {
        println!("Casting Fireball!");
    }
}

impl Ability for Heal {
    fn use_ability(&self) {
        println!("Casting Heal!");
    }
}

// Liskov Substitution Principle (LSP) Example
// This code demonstrates a proper implementation of the Liskov Substitution Principle (LSP).

trait Mount {
    fn ride(&self);
}

struct Horse;

impl Mount for Horse {
    fn ride(&self) {
        println!("Riding the horse!");
    }
}

fn mount_ride<T: Mount>(mount: T) {
    mount.ride();
}

// Interface Segregation Principle (ISP) Example
// This code demonstrates a proper implementation of the Interface Segregation Principle (ISP).

trait Fighter {
    fn attack(&self);
}

trait Healer {
    fn heal(&self);
}

struct Paladin;

impl Fighter for Paladin {
    fn attack(&self) {
        println!("Paladin attacks with a sword!");
    }
}

impl Healer for Paladin {
    fn heal(&self) {
        println!("Paladin heals with a blessing!");
    }
}

// Dependency Inversion Principle (DIP) Example
// This code demonstrates a proper implementation of the Dependency Inversion Principle (DIP).

trait Weapon {
    fn use_weapon(&self);
}

struct Sword;

impl Weapon for Sword {
    fn use_weapon(&self) {
        println!("Swinging the sword!");
    }
}

struct Axe;

impl Weapon for Axe {
    fn use_weapon(&self) {
        println!("Swinging the axe!");
    }
}

struct Warrior<T: Weapon> {
    weapon: T,
}

impl<T: Weapon> Warrior<T> {
    fn fight(&self) {
        self.weapon.use_weapon();
    }
}
