
macro_rules! magic_spelling {
    (fire) => {
        println!("Casting Fireball!");
    };
    (water) => {
        println!("Casting Water Shield!");
    }
}

fn main() {
    magic_spelling!(fire);
    magic_spelling!(water);
}
