pub mod grid {
    #[derive(Clone, Copy, Eq, PartialEq, Debug)]
    pub enum Cell {
        Digit(u8),
        Empty,
    }

    #[derive(Debug)]
    pub struct Table {
        pub grid: [[Cell; 9]; 9],
    }

    impl Table {
        pub fn new(grid: [[Cell; 9]; 9]) -> Table {
            Table { grid }
        }

        pub fn from(other: &Table) -> Table {
            other.clone()
        }

        pub fn empty_cells(&self) -> Vec<(usize, usize)> {
            let mut res = vec![];
            for r in 0..9 {
                for c in 0..9 {
                    if self.grid[r][c] == Cell::Empty {
                        res.push((r, c));
                    }
                }
            }
            res
        }

        pub fn row_is_ok(&self, r: usize) -> bool {
            assert!(r < 9);
            let mut seen = 0;
            for c in 0..9 {
                if let Cell::Digit(d) = self.grid[r][c] {
                    if (seen & (1 << d)) != 0 {
                        return false;
                    }
                    seen |= 1 << d;
                }
            }
            true
        }

        pub fn col_is_ok(&self, c: usize) -> bool {
            assert!(c < 9);
            let mut seen = 0;
            for r in 0..9 {
                if let Cell::Digit(d) = self.grid[r][c] {
                    if (seen & (1 << d)) != 0 {
                        return false;
                    }
                    seen |= 1 << d;
                }
            }
            true
        }

        // (0,0) | (0,1) | (0,2)
        // ------|-------|------
        // (1,0) | (1,1) | (1,2)
        // ------|-------|------
        // (2,0) | (2,1) | (2,2)
        pub fn group_is_ok(&self, gr: usize, gc: usize) -> bool {
            assert!(gr <= 2 && gc <= 2);
            let mut seen = 0;
            for r in (3 * gr)..(3 * gr + 3) {
                for c in (3 * gc)..(3 * gc + 3) {
                    if let Cell::Digit(d) = self.grid[r][c] {
                        if (seen & (1 << d)) != 0 {
                            return false;
                        }
                        seen |= 1 << d;
                    }
                }
            }
            true
        }
    }

    impl Default for Table {
        fn default() -> Self {
            Table { grid: [[Cell::Empty; 9]; 9] }
        }
    }

    impl Clone for Table {
        fn clone(&self) -> Self {
            Table { grid: self.grid.clone() }
        }
    }
}

pub mod solver {}