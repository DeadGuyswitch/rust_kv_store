use core::panic;

pub struct KvStore {
    key: String,
    value: String
}

impl KvStore {

    pub fn new() -> Self {
        panic!();
    }

    pub fn set(&mut self, key:String, value: String) {
        panic!();
    }

    pub fn get(&self, key:String) -> Option<String>{
        panic!();
    }

    pub fn remove(&mut self, key:String) {
        panic!();
    }
}