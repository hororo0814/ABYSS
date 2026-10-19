#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponNote {
    id: u32,
    name: String,
    description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inventory {
    weapons: Vec<WeaponNote>,
}

impl WeaponNote {
    pub fn new(id: u32, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
        }
    }
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            weapons: Vec::new(),
        }
    }

    pub fn find_id(&self, id: u32) -> bool {
        self.weapons.iter().any(|target| target.id == id)
    }

    pub fn add_weapon(&mut self, id: u32, name: String, description: String) -> bool {
        if self.find_id(id) {
            return false;
        }
        let new_weapon = WeaponNote::new(id, name, description);
        self.weapons.push(new_weapon);
        true
    }

    pub fn weapon_count(&self) -> usize {
        self.weapons.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::inventory::Inventory;

    #[test]
    fn empty_weapons_test() {
        let new_weapons = Inventory::new();
        assert_eq!(new_weapons.find_id(1), false);
    }

    #[test]
    fn add_weapons_test() {
        let mut new_weapons = Inventory::new();
        new_weapons.add_weapon(1, "first".to_string(), "this_is_first_item".to_string());
        assert_eq!(new_weapons.find_id(1), true);
    }

    #[test]
    fn add_weapons_multiple_test() {
        let mut new_weapons = Inventory::new();
        new_weapons.add_weapon(1, "first".to_string(), "this_is_first_item".to_string());
        new_weapons.add_weapon(1, "first".to_string(), "this_is_first_item".to_string());
        assert_eq!(new_weapons.weapons.len(), 1);
    }
}
