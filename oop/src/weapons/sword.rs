use super::weapon::Weapon;

pub struct Sword;

impl Weapon for Sword {
    fn attack(&self) {
        println!("Swinging the sword!");
    }
}