// Traits like a set of abilities or behaviors it define shared functionalities
// that multiple structs or enums that can implement.
// (It's look like interfaces in other languages)

// Why we use traits?
// avoid duplicate code for common behaviors
// allow different structs to share functionality
// ensure consistent behavior across different characters

trait TreasureCollector {
    fn collect_treasure(&self);
}

struct Crabby {
    name: String,
}

impl TreasureCollector for Crabby {
    fn collect_treasure(&self) {
       println!("{} is collecting treasures!", self.name);
    }
}

// Generics let's functions or structs work with multiple types instead of just one.
// generic parameter <T>

fn collect_any_treasure<T: std::fmt::Debug>(treasure: T) {
    println!("You collected some teasure: {:?}", treasure);
}

trait DisplayItem {
    fn display_item(&self);
}

struct Inventory<T> {
    item: T,
}

// impl<T: std::fmt::Debug> DisplayItem for Inventory<T> {
//     fn display_item(&self) {
//        println!("Inventory contains: {:?}", self.item);
//     }
// }

// or this style
impl<T> DisplayItem for Inventory<T>
where
    T: std::fmt::Debug,
{
    fn display_item(&self) {
        println!("Inventory contains: {:?}", self.item);
    }
}

fn main() {
    let crabby = Crabby { name: String::from("Crabby") };
    crabby.collect_treasure();

    collect_any_treasure(10);
    collect_any_treasure("Diamonds");

    let gold_inventory = Inventory { item: 100 };
    let map_inventory = Inventory { item: String::from("Treasure Map") };

    println!("Holding {} gold coins.", gold_inventory.item);
    println!("Holding a {}.", map_inventory.item);

    gold_inventory.display_item();
    map_inventory.display_item();
}
