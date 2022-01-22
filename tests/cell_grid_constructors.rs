#[cfg(test)]
mod tests {
    use sudoku_solver_ed::grid::{Cell, Table};

    #[test]
    fn cell_constructor_works() {
        let a = Cell::Digit(2);
        let mut b = Cell::Digit(9);
        assert_eq!(a, Cell::Digit(2));
        assert_eq!(b, Cell::Digit(9));
        b = a.clone();
        assert_eq!(a, Cell::Digit(2));
        assert_eq!(b, Cell::Digit(2));
        b = Cell::Digit(8);
        assert_eq!(a, Cell::Digit(2));
        assert_eq!(b, Cell::Digit(8));
    }

    #[test]
    fn grid_constructor_works() {
        let mut a: Table = Default::default();
        let b = Table::new([[Cell::Digit(1); 9]; 9]);
        let mut c = Table::from(&b);
        assert_eq!(a.grid[8][8], Cell::Empty);
        assert_eq!(b.grid[3][4], Cell::Digit(1));
        assert_eq!(c.grid[2][1], Cell::Digit(1));
        a.grid[0][0] = Cell::Digit(2);
        assert_eq!(a.grid[0][0], Cell::Digit(2));
        assert_eq!(a.grid[8][8], Cell::Empty);
        c.grid[2][1] = Cell::Empty;
        assert_eq!(b.grid[2][1], Cell::Digit(1));
        assert_eq!(c.grid[2][1], Cell::Empty);
        let d = Table::from_arr([[2, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0]]);
        assert_eq!(a, d);
    }
}