enum CrabbyState {
    Resting,
    Fighting,
    Collecting(u32),
    Defending,
}

impl CrabbyState {
    fn describe(&self) {
        match self {
            CrabbyState::Resting => println!("Crabby is resting."),
            CrabbyState::Fighting => println!("Crabby is fighting bad crabs."),
            CrabbyState::Collecting(amount) => println!("Crabby is collecting {} treasures.", amount),
            CrabbyState::Defending => println!("Crabby is defending this treasure.")
        }
    }
}

enum CrabbyActionStates {
    Fighting,
    Collecting(u32),
    Defending,
}

impl CrabbyActionStates {
    fn state_represent(&self) {
        match self {
            CrabbyActionStates::Fighting => println!("Crabby is fighting!"),
            CrabbyActionStates::Collecting(amount) => println!("Crabby is collecting {} treasures.", amount),
            CrabbyActionStates::Defending => println!("Crabby is defending this treasure.")
        };
    }
}

fn main() {
    let current_state = CrabbyState::Resting;

    match current_state {
        CrabbyState::Resting => println!("Crabby is resting."),
        CrabbyState::Fighting => println!("Crabby is fighting bad crabs."),
        CrabbyState::Collecting(amount) => println!("Crabby is collecting {} treasures.", amount),
        CrabbyState::Defending => println!("Crabby is defending this treasure.")
    }
    
    let fighting = CrabbyActionStates::Fighting;
    let collecting = CrabbyActionStates::Collecting(15);
    let defending = CrabbyActionStates::Defending;
    
    fighting.state_represent();
    collecting.state_represent();
    defending.state_represent();
}
