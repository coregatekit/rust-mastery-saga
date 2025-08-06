pub mod challenge;

use challenge::{Warrior, Mage, Healer};

use crate::challenge::challenge::special_attack;

pub fn app() {
    let mut warrior = Warrior::new();
    let mut mage = Mage::new();
    let mut healer = Healer::new();

    warrior.health_decrease(10);
    mage.health_decrease(10);
    healer.health_decrease(10);

    println!("Warrior Health: {}", warrior.health);
    println!("Mage Health: {}", mage.health);
    println!("Healer Health: {}", healer.health);

    warrior.health_increase(10);
    mage.health_increase(10);
    healer.health_increase(10);

    println!("Warrior Health: {}", warrior.health);
    println!("Mage Health: {}", mage.health);
    println!("Healer Health: {}", healer.health);

    special_attack(warrior.weapon);
    special_attack(mage.weapon);
    special_attack(healer.weapon);
}