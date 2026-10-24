use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct EnemyNote {
    id: u32,
    name: String,
    description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnemyInventory {
    memo_enemies: Vec<EnemyNote>,
}

impl Default for EnemyInventory {
    fn default() -> Self {
        Self::new()
    }
}

impl EnemyNote {
    pub fn new(id: u32, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

impl EnemyInventory {
    pub fn new() -> Self {
        Self {
            memo_enemies: Vec::new(),
        }
    }

    //idからそのウェポンの中身を取り出せるようにする
    pub fn get_enemy(&self, id: u32) -> Option<&EnemyNote> {
        self.memo_enemies.iter().find(|weapon| weapon.id == id)
    }

    //可変参照でidから取り出せるゲッター
    pub fn get_enemy_mut(&mut self, id: u32) -> Option<&mut EnemyNote> {
        self.memo_enemies.iter_mut().find(|weapon| weapon.id == id)
    }

    pub fn find_id(&self, id: u32) -> bool {
        self.memo_enemies.iter().any(|target| target.id == id)
    }

    pub fn add_enemy(&mut self, id: u32, name: String, description: String) -> bool {
        if self.find_id(id) {
            return false;
        }
        let new_enemy = EnemyNote::new(id, name, description);
        self.memo_enemies.push(new_enemy);
        true
    }

    pub fn enemy_count(&self) -> usize {
        self.memo_enemies.len()
    }
}
