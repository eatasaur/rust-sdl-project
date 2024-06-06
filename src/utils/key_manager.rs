use std::collections::HashMap;

pub struct KeyManager {
    keys: HashMap<String, bool>
}

impl KeyManager {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new()
        }
    }

    pub fn key_down(&mut self, key_name: String) {
        if !self.keys.contains_key(&key_name) {
            self.keys.entry(key_name).or_insert(true);
        }
        else {
            if let Some(x) = self.keys.get_mut(&key_name) {
                *x = true;
            }
        }
    }

    pub fn key_up(&mut self, key_name: String) {
        if !self.keys.contains_key(&key_name) {
            self.keys.entry(key_name).or_insert(false);
        }
        else {
            if let Some(x) = self.keys.get_mut(&key_name) {
                *x = false;
            }
        }
    }

    pub fn is_key_pressed(&self, value: &str) -> bool {
        self.keys.contains_key(&value.to_string()) && self.keys.get(&value.to_string()) == Some(&true)
    }
}