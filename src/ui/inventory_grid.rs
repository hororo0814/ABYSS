pub const GRID_COLUMNS: u32 = 4;
pub const GRID_ROWS: u32 = 7;
pub const MAX_WEAPON_ID: u32 = GRID_COLUMNS * GRID_ROWS;

pub fn grid_position(id: u32) -> Option<(u32, u32)> {
    if !(1..=MAX_WEAPON_ID).contains(&id) {
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
    fn first_weapon_is_at_top_left() {
        assert_eq!(grid_position(1), Some((0, 0)));
    }

    #[test]
    fn sixth_weapon_is_in_second_row_second_column() {
        assert_eq!(grid_position(6), Some((1, 1)));
    }

    #[test]
    fn last_weapon_is_at_bottom_right() {
        assert_eq!(grid_position(28), Some((3, 6)));
    }

    #[test]
    fn invalid_id_has_no_position() {
        assert_eq!(grid_position(0), None);
        assert_eq!(grid_position(29), None);
    }
}
