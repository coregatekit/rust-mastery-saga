// struct Crabby {
//     name: String,
//     skill_level: u32,
//     hit_points: i32,
// }
//
// impl Crabby {
//     fn take_damage(&mut self, damage: i32) { // self keyword is reference to current instance that called this method
//         self.hit_points -= damage;
//         println!("{} takes {} damage, hit points now at: {}", self.name, damage, self.hit_points);
//     }
// }
//
// fn main() {
//     let mut crabby = Crabby {
//         name: String::from("Crabby"),
//         skill_level: 10,
//         hit_points: 100,
//     };
//
//     crabby.hit_points = 80;
//     println!("{} has skill level {} and {} hit points", crabby.name, crabby.skill_level, crabby.hit_points);
//
//     crabby.take_damage(15);
//
//     let crabby = Crabby {
//         name: String::from("Crabby"),
//         skill_level: 10,
//         hit_points: 100,
//     };
//
//     let new_crabby = Crabby {
//         name: String::from("Hero Crabby"),
//         ..crabby // make a clone of crabby but change only name beware of heap data it will change owner
//     };
//
//     println!("{} with skill level {}", new_crabby.name, new_crabby.skill_level);
// }
//

struct Crabby {
    name: String,
    health: u8, // max 100
}

impl Crabby {
    fn take_damage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    fn healing(&mut self, heal: u8) {
        if self.health + heal >= 100 {
            self.health = 100;
            return;
        }
        self.health += heal;
    }
}

fn main() {
    let mut crabby = Crabby{
        name: String::from("Crabby"),
        health: 100,
    };

    crabby.take_damage(100);
    println!("{} health: {}", crabby.name, crabby.health);
    crabby.take_damage(10);
    println!("{} health: {}", crabby.name, crabby.health);

    crabby.healing(60);
    println!("{} health: {}", crabby.name, crabby.health);
}