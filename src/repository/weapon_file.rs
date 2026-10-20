use crate::game_data::GameData;
use crate::models::inventory::WeaponNote;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

//プレイヤー入力のウェポンを読み込む
pub fn load_weapons(path: impl AsRef<Path>) -> Result<Vec<WeaponNote>, String> {
    let contents = fs::read_to_string(path)
        .map_err(|error| format!("Weaponファイルの読み込みに失敗しました: {error}"))?;

    let player_weapons: Vec<WeaponNote> = serde_json::from_str(&contents)
        .map_err(|error| format!("Weaponデータの解析に失敗しました: {error}"))?;

    validate_weapons(&player_weapons)?;
    Ok(player_weapons)
}

fn validate_weapons(weapons: &[WeaponNote]) -> Result<(), String> {
    let mut ids = HashSet::new();

    for weapon in weapons {
        let id = weapon.id();

        if !(1..=28).contains(&id) {
            return Err(format!("武器 ID が範囲外です: {id}"));
        }

        if !ids.insert(id) {
            return Err(format!("武器 ID が重複しています: {id}"));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_weapons_from_json() {
        let weapons = load_weapons("data/player_weapons.json").unwrap();

        assert_eq!(weapons.len(), 2);
        assert_eq!(weapons[0].id(), 1);
        assert_eq!(weapons[0].name(), "playerの入力する名前");
    }

    #[test]
    fn rejects_weapon_id_out_of_range() {
        let json = r#"
        [
            {
                "id": 89,
                "name": "playerの入力する名前",
                "description": "playerの入力する説明"
            }
        ]
        "#;

        let player_weapons: Vec<WeaponNote> = serde_json::from_str(json).unwrap();

        assert!(validate_weapons(&player_weapons).is_err());
    }
}
