use crate::composition::{Armor, Weapon};

use super::Crabby;

pub fn composition() {
    let crabby = Crabby {
      weapons: vec![Weapon{
        name: "Claw".to_string(),
        damage: 10,
      }],
      armors: vec![Armor{
        name: "Shell".to_string(),
        defense: 5,
      }],
    };

    println!("{:?}", crabby);
}