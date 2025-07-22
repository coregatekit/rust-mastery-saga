fn main() {
    let total_apples = count_apples(5);
    println!("Crabby has collected {} apples!", total_apples);
    
    let result = crabby_tasks("gatering coins", 12);
    println!("{}", result);
    let result2 = crabby_tasks("preparing sword", 20);
    println!("{}", result2);
    let result3 = crabby_tasks("change clothes", 18);
    println!("{}", result3);
}

fn count_apples(n: i32) -> i32 {
    n + 2
}

fn crabby_tasks(task: &str, time: i32) -> String {
    format!("Crabby has successfully done {} in {} minutes", task, time)
}
