pub mod grid {
    #[derive(Clone, Copy, Eq, PartialEq, Debug)]
    pub enum Cell {
        Digit(u8),
        Empty,
    }

    #[derive(Debug, Eq, PartialEq)]
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

        pub fn from_arr(arr: [[u8; 9]; 9]) -> Table {
            let mut grid: [[Cell; 9]; 9] = Default::default();
            for i in 0..9 {
                for j in 0..9 {
                    assert!(arr[i][j] < 10);
                    if arr[i][j] == 0 {
                        grid[i][j] = Cell::Empty;
                    } else {
                        grid[i][j] = Cell::Digit(arr[i][j]);
                    }
                }
            }
            Table { grid }
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

        pub fn solved(&self) -> bool {
            for i in 0..9 {
                if !self.row_is_ok(i) || !self.col_is_ok(i) {
                    return false;
                }
            }
            for gr in 0..3 {
                for gc in 0..3 {
                    if !self.group_is_ok(gr, gc) {
                        return false;
                    }
                }
            }
            self.empty_cells().is_empty()
        }
    }

    impl Default for Cell {
        fn default() -> Self {
            Cell::Empty
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

pub mod solver {
    // use crate::grid::{Cell, Table};
    //
    // pub fn solve_dfs_single(t: Table) -> Table {
    //     let emptys = t.empty_cells();
    //
    //     assert!(t.solved());
    //     t
    // }
}