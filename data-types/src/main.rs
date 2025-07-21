fn main() {
    // Scalar types

    let x = 5; // dynamic typing as i32
    let y = 10.5; // dynamic typing as f64

    let sum = x + y as i32; // casting type y to i32
    println!("sum is {}", sum);
    
    // Flexible text type
    // String store in heap (dynamic size)
    // &str store in stack (fixed size at runtime)

    let message = String::from("Hello, Crabby!");
    println!("message is {}", message);
    
    let msg1: String = String::from("Hello, world!");
    let msg2: String = "Hello, Crabby!".to_string();
    let msg3: &str = "Hello, World!";
    let msg4 = format!("{}, {}", x, y);

    println!("msg1 is {}", msg1);
    println!("msg2 is {}", msg2);
    println!("msg3 is {}", msg3); 
    println!("msg4 is {}", msg4);
}
