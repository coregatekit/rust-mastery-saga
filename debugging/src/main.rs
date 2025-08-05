fn main() {
    let x = 5;
    let y = 10;
    let r = call_level_1(x, y);

    println!("The result is: {}", r);
}

fn call_level_1(x: i32, y: i32) -> i32 {
    let z = 15 + x;
    let w = 20 + y;
    return call_level_2(z, w);
}

fn call_level_2(a: i32, b: i32) -> i32 {
    let m = 25 + a;
    let n = 30 + b;
    return m + n;
}
