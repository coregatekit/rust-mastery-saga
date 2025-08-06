pub mod app;

struct Crabby;
struct MageFriend;

trait Attack {
    fn attack(&self);
}

impl Attack for Crabby {
    fn attack(&self) {
        println!("Crabby attacks with claws!");
    }
}

impl Attack for MageFriend {
    fn attack(&self) {
        println!("MageFriend casts a spell!");
    }
}

pub fn do_attack<T: Attack>(attacker: T) { // <- Different objects can do the same action in there own unique way
    attacker.attack();
}
