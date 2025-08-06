// We use composition to create a Crabby struct that can hold weapons and armors.
// Like inheritance, composition allows us to build complex types from simpler ones.

pub mod app;

#[derive(Debug)]
struct Crabby {
  // Composition with armors and weapons
  weapons: Vec<Weapon>,
  armors: Vec<Armor>,
}

#[derive(Debug)]
struct Weapon {
  name: String,
  damage: i32,
}

#[derive(Debug)]
struct Armor {
  name: String,
  defense: i32,
}