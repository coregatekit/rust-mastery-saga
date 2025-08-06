use oop::{
    challenge, composition, crabby::Crabby, polymorphism, weapons::{bow::Bow, sword::Sword, weapon::Weapon}
};

fn main() {
    println!("Hello, world!");

    let sword = Sword;
    let bow = Bow;

    sword.attack();
    bow.attack();

    let mut crabby = Crabby::new();
    crabby.add("seashell".to_string());
    crabby.add("pebble".to_string());

    let crabby_backpack = crabby.open();
    println!("Crabby's backpack contains: {:?}", crabby_backpack);

    composition::app::composition();

    polymorphism::app::app();

    println!("==================================================================");
    challenge::app();
}
