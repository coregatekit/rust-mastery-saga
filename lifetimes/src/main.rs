fn main() {
    let treasure;
    {
        let local_treasure = String::from("Gold coins");
        treasure = &local_treasure;
        println!("treasure: {}", treasure);
    }
    // println!("treasure: {}", treasure); // treasure is not live long enough

    let treasure1 = "Gold coins"; // <- treasure1 was killed
    let treasure2 = "Silver coins";

    let result = longest_treasure(treasure1, treasure2);
    println!("The longest treasure result is {}", result);
    
    let map1 = "Ancient Map of the Sea";
    let map2 = "Map to Hidden Gold";
    
    let chosen_map = longest_map(map1, map2);
    println!("The longest chosen map is {}", chosen_map);
}

// fn longest_treausure(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x // <- x was killed
//     } else {
//         y
//     }
// }

// to fix missing lifetimes just add generic 'a
// <'a> &'a
// The lifetime annotation 'a ensures both x and y will be the same life time (equal to who called this function)
fn longest_treasure<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x // <- x was killed
    } else {
        y
    }
}

fn longest_map<'a>(map1: &'a str, map2: &'a str) -> &'a str {
    if map1.len() > map2.len() {
        map1
    } else {
        map2
    }
}
