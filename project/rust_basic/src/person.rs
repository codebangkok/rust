use crate::speaking::Speaking;

pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Self {
            name: name,
            age: age,
        }
    }

    pub fn hello(&self) {
        println!("My name is {}", self.name);
    }
}

impl Speaking for Person {
    fn speak(&self) {
        println!("{} Speaking...", self.name);
    }
}
