// Iterators
// An iterator is something that lets us traverse over a collection,
// like a vector or array, one item at a time

fn main() {
    let treasures = vec!["Gold", "Gem", "Sword"];
    
    for treasure in treasures {
        println!("{}", treasure);
    }
    
    let treasures = vec![100, 200, 300, 400];
    
    // |x| x * 2 -> this is closure, inside | | is parameter
    let doubled: Vec<i32> = treasures.iter().map(|x| x * 2).collect();
    
    // iter 1: x is the first index of treasure
    // treasures: [100, 200, 300, 400]
    // in closure |100| 100 * 2 = 200
    // doubled: [200, 200, 300, 400]
    // ...
    // iter 4:
    // treasures: [100, 200, 300, 400]
    // doubled: [200, 400, 600, 800]
    println!("{:?}", doubled);
    
    let add = |a, b| a + b;
    // looks like short function
    // fn add(a: i32, b: i32) -> i32 {
    //     a + b
    // }
    let result = add(3, 4);
    println!("{}", result);
    
    println!("===========================================");
    challenge();
}

fn challenge() {
    let treasures = vec![100, 200, 300, 400];
    
    let doubled_treasures: Vec<i32> = treasures.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled_treasures);
}
