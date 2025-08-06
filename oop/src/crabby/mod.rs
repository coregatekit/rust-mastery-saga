pub struct Crabby {
    backpack: Vec<String>,
}

impl Crabby {
    pub fn new() -> Self {
        Crabby { backpack: vec![] }
    }

    pub fn add(&mut self, item: String) {
        self.backpack.push(item);
    }

    pub fn open(&self) -> Vec<String> {
        self.backpack.clone()
    }
}
