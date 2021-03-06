#[cfg(test)]
mod tests {
    use sudoku_solver_ed::grid::{Cell, Table};

    #[test]
    fn empty_cells_works() {
        let mut a: Table = Default::default();
        assert_eq!(a.empty_cells().len(), 81);
        a.grid[8][7] = Cell::Digit(2);
        assert_eq!(a.empty_cells().len(), 80);
        a.grid[8][7] = Cell::Empty;
        let cells = a.empty_cells();
        assert_eq!(cells.len(), 81);
        for i in 0..cells.len() {
            assert_eq!(cells[i], (i / 9, i % 9));
        }
    }

    #[test]
    fn ok_row_works() {
        let mut a: Table = Default::default();
        for i in 0..9 {
            assert!(a.row_is_ok(0));
            a.grid[0][i] = Cell::Digit(i as u8 + 1);
            assert!(a.row_is_ok(i));
        }
        assert!(a.row_is_ok(0));
        for i in 0..9 {
            if a.grid[0][i] != Cell::Digit(4) {
                a.grid[0][i] = Cell::Digit(4);
                assert!(!a.row_is_ok(0));
            }
            a.grid[0][i] = Cell::Digit(i as u8 + 1);
            assert!(a.row_is_ok(0));
        }
    }

    #[test]
    fn ok_col_works() {
        let mut a: Table = Default::default();
        for i in 0..9 {
            assert!(a.col_is_ok(8));
            a.grid[i][8] = Cell::Digit(i as u8 + 1);
            assert!(a.col_is_ok(i));
        }
        assert!(a.col_is_ok(8));
        for i in 0..9 {
            if a.grid[i][8] != Cell::Digit(4) {
                a.grid[i][8] = Cell::Digit(4);
                assert!(!a.col_is_ok(8));
            }
            a.grid[i][8] = Cell::Digit(i as u8 + 1);
            assert!(a.col_is_ok(8));
        }
    }

    #[test]
    fn ok_group_works() {
        let mut a = Table::new([[Cell::Digit(3); 9]; 9]);
        for gr in 0..3 {
            for gc in 0..3 {
                assert!(!a.group_is_ok(gr, gc));
            }
        }
        a.grid[3][3] = Cell::Digit(1);
        assert!(!a.group_is_ok(1, 1));
        a.grid[3][4] = Cell::Digit(2);
        assert!(!a.group_is_ok(1, 1));
        a.grid[3][5] = Cell::Digit(3);
        assert!(!a.group_is_ok(1, 1));
        a.grid[4][3] = Cell::Digit(4);
        assert!(!a.group_is_ok(1, 1));
        a.grid[4][4] = Cell::Digit(5);
        assert!(!a.group_is_ok(1, 1));
        a.grid[4][5] = Cell::Digit(6);
        assert!(!a.group_is_ok(1, 1));
        a.grid[5][3] = Cell::Digit(7);
        assert!(!a.group_is_ok(1, 1));
        a.grid[5][4] = Cell::Digit(8);
        assert!(!a.group_is_ok(1, 1));
        a.grid[5][5] = Cell::Digit(9);
        assert!(a.group_is_ok(1, 1));
    }

    #[test]
    fn solved_works() {
        let mut a: Table = Default::default();
        assert!(!a.solved());
        a = Table::from_arr([
            [4, 3, 5, 2, 6, 9, 7, 8, 1],
            [6, 8, 2, 5, 7, 1, 4, 9, 3],
            [1, 9, 7, 8, 3, 4, 5, 6, 2],
            [8, 2, 6, 1, 9, 5, 3, 4, 7],
            [3, 7, 4, 6, 8, 2, 9, 1, 5],
            [9, 5, 1, 7, 4, 3, 6, 2, 8],
            [5, 1, 9, 3, 2, 6, 8, 7, 4],
            [2, 4, 8, 9, 5, 7, 1, 3, 6],
            [7, 6, 3, 4, 1, 8, 2, 5, 9],
        ]);
        assert!(a.solved());
        a.grid[8][8] = Cell::Empty;
        assert!(!a.solved());
    }
}
