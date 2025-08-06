pub trait Weapon {
    fn attack(&self) {}
}

pub struct Sword;

impl Weapon for Sword {
    fn attack(&self) {
        println!("Swinging sword!");
    }
}

pub struct Staff;

impl Weapon for Staff {
    fn attack(&self) {
        println!("Casting spell with staff!");
    }
}

pub struct Warrior {
    pub health: i32,
    pub intelligence: i32,
    pub strength: i32,
    pub weapon: Box<dyn Weapon>,
}

impl Warrior {
    pub fn new() -> Self {
        Self {
            health: 100,
            intelligence: 0,
            strength: 10,
            weapon: Box::new(Sword),
        }
    }

    pub fn health_increase(&mut self, value: i32) {
        if self.health + value > 100 {
            self.health = 100;
        } else {
            self.health += value
        }
    }

    pub fn health_decrease(&mut self, value: i32) {
        self.health = self.health.saturating_sub(value);
    }
}

pub struct Mage {
    pub health: i32,
    pub intelligence: i32,
    pub strength: i32,
    pub weapon: Box<dyn Weapon>,
}

impl Mage {
    pub fn new() -> Self {
        Self {
            health: 100,
            intelligence: 10,
            strength: 0,
            weapon: Box::new(Staff),
        }
    }

    pub fn health_increase(&mut self, value: i32) {
        if self.health + value > 100 {
            self.health = 100;
        } else {
            self.health += value
        }
    }

    pub fn health_decrease(&mut self, value: i32) {
        self.health = self.health.saturating_sub(value);
    }
}

pub struct Healer {
    pub health: i32,
    pub intelligence: i32,
    pub strength: i32,
    pub weapon: Box<dyn Weapon>,
}

impl Healer {
    pub fn new() -> Self {
        Self {
          health: 100,
          intelligence: 5,
          strength: 5,
          weapon: Box::new(Staff),
        }
    }

    pub fn health_increase(&mut self, value: i32) {
        if self.health + value > 100 {
            self.health = 100;
        } else {
            self.health += value
        }
    }

    pub fn health_decrease(&mut self, value: i32) {
        self.health = self.health.saturating_sub(value);
    }
}

pub fn special_attack(weapon: Box<dyn Weapon>) {
    weapon.attack();
}
