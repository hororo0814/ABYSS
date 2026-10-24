pub const GRID_COLUMNS: u32 = 3;
pub const GRID_ROWS: u32 = 10;
pub const MAX_ENEMY_ID: u32 = GRID_COLUMNS * GRID_ROWS;

pub fn grid_position(id: u32) -> Option<(u32, u32)> {
    if !(1..=MAX_ENEMY_ID).contains(&id) {
        return None;
    }

    let index = id - 1;
    let column = index % GRID_COLUMNS;
    let row = index / GRID_COLUMNS;

    Some((column, row))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_enemy_is_at_top_left() {
        assert_eq!(grid_position(1), Some((0, 0)));
    }

    #[test]
    fn sixth_enemy_is_in_second_row_second_column() {
        assert_eq!(grid_position(5), Some((1, 1)));
    }

    #[test]
    fn last_enemy_is_at_bottom_right() {
        assert_eq!(grid_position(30), Some((2, 9)));
    }

    #[test]
    fn invalid_enemy_id_has_no_position() {
        assert_eq!(grid_position(0), None);
        assert_eq!(grid_position(31), None);
    }
}
