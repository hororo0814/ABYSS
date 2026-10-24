use crate::models::enemy_inventory::EnemyNote;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

//プレイヤー入力のウェポンを読み込む
pub fn load_enemies(path: impl AsRef<Path>) -> Result<Vec<EnemyNote>, String> {
    let contents = fs::read_to_string(path)
        .map_err(|error| format!("Enemyファイルの読み込みに失敗しました: {error}"))?;

    let memo_enemies: Vec<EnemyNote> = serde_json::from_str(&contents)
        .map_err(|error| format!("Enemyデータの解析に失敗しました: {error}"))?;

    validate_enemies(&memo_enemies)?;
    Ok(memo_enemies)
}

fn validate_enemies(enemies: &[EnemyNote]) -> Result<(), String> {
    let mut ids = HashSet::new();

    for enemy in enemies {
        let id = enemy.id();

        if !(1..=30).contains(&id) {
            return Err(format!("Enemy ID が範囲外です: {id}"));
        }

        if !ids.insert(id) {
            return Err(format!("Enemy ID が重複しています: {id}"));
        }
    }

    Ok(())
}
