use super::weapon::Weapon;

pub struct Bow;

impl Weapon for Bow {
    fn attack(&self) {
        println!("Shooting an arrow with the bow!");
    }
}