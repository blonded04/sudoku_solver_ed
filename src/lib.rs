pub mod grid {
    use std::io;

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

        pub fn read() -> Result<Table, &'static str> {
            let mut grid: [[u8; 9]; 9] = Default::default();
            for r in 0..9 {
                let mut buf = String::new();
                if let Err(_e) = io::stdin().read_line(&mut buf) {
                    return Err("Couldn't read from stdin.");
                }
                let digits: Vec<u32> = buf
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                if digits.len() != 9 {
                    return Err("Please follow input format.");
                }
                for c in 0..9 {
                    grid[r][c] = digits[c] as u8;
                }
            }
            Ok(Table::from_arr(grid))
        }

        pub fn print(&self) {
            for r in 0..9 {
                for c in 0..9 {
                    let v;
                    if let Cell::Digit(d) = self.grid[r][c] {
                        v = d;
                    } else {
                        v = 0;
                    }
                    if c != 8 {
                        print!(" {} |", v);
                    } else {
                        println!(" {} ", v);
                    }
                }
                if r != 9 {
                    println!("-----------------------------------");
                }
            }
        }
    }

    impl Default for Cell {
        fn default() -> Self {
            Cell::Empty
        }
    }

    impl Default for Table {
        fn default() -> Self {
            Table {
                grid: [[Cell::Empty; 9]; 9],
            }
        }
    }

    impl Clone for Table {
        fn clone(&self) -> Self {
            Table {
                grid: self.grid.clone(),
            }
        }
    }
}

pub mod solver {
    use crate::grid::{Cell, Table};
    use rand::seq::SliceRandom;
    use rand::{thread_rng, Rng};

    pub const ANNEALING_ITERS: u32 = 130000;

    fn dfs(t: &mut Table, i: usize, emptys: &Vec<(usize, usize)>) -> bool {
        if emptys.len() <= i {
            return true;
        }
        for d in 1..=9 {
            let (row, col) = emptys[i];
            t.grid[row][col] = Cell::Digit(d);
            if t.row_is_ok(row)
                && t.col_is_ok(col)
                && t.group_is_ok(row / 3, col / 3)
                && dfs(t, i + 1, emptys)
            {
                return true;
            }
        }
        t.grid[emptys[i].0][emptys[i].1] = Cell::Empty;
        false
    }

    fn dfs_rand(t: &mut Table, i: usize, emptys: &Vec<(usize, usize)>) -> bool {
        if emptys.len() <= i {
            return true;
        }
        let mut order: Vec<u8> = (1..=9).collect();
        order.shuffle(&mut thread_rng());
        for d in order {
            let (row, col) = emptys[i];
            t.grid[row][col] = Cell::Digit(d);
            if t.row_is_ok(row)
                && t.col_is_ok(col)
                && t.group_is_ok(row / 3, col / 3)
                && dfs(t, i + 1, emptys)
            {
                return true;
            }
        }
        t.grid[emptys[i].0][emptys[i].1] = Cell::Empty;
        false
    }

    pub fn solve_dfs_single(mut t: Table) -> Result<Table, &'static str> {
        let emptys = t.empty_cells();
        if dfs(&mut t, 0, &emptys) {
            Ok(t)
        } else {
            Err("No solution found")
        }
    }

    pub fn solve_randomized_dfs_single(mut t: Table) -> Result<Table, &'static str> {
        let mut emptys = t.empty_cells();
        emptys.shuffle(&mut thread_rng());
        if dfs_rand(&mut t, 0, &emptys) {
            Ok(t)
        } else {
            Err("No solution found")
        }
    }

    fn generate_digits(sz: usize) -> Vec<u8> {
        let mut res = vec![];
        res.reserve(sz);
        for _i in 0..sz {
            res.push(thread_rng().gen_range(1..=9));
        }
        res
    }

    fn apply(t: &mut Table, emptys: &Vec<(usize, usize)>, values: &Vec<u8>) {
        assert_eq!(emptys.len(), values.len());
        for i in 0..emptys.len() {
            t.grid[emptys[i].0][emptys[i].1] = Cell::Digit(values[i]);
        }
    }

    fn inversions(t: &Table) -> i16 {
        let mut res = 0;
        for i in 0..9 {
            if !t.row_is_ok(i) {
                res += 1;
            }
            if !t.col_is_ok(i) {
                res += 1;
            }
        }
        for gr in 0..3 {
            for gc in 0..3 {
                if !t.group_is_ok(gr, gc) {
                    res += 1;
                }
            }
        }
        res
    }

    fn calc_change(t: &mut Table, r: usize, c: usize, d: u8) -> i16 {
        let old = t.grid[r][c];

        let mut had = 0;
        if !t.row_is_ok(r) {
            had += 1;
        }
        if !t.col_is_ok(c) {
            had += 1;
        }
        if !t.group_is_ok(r / 3, c / 3) {
            had += 1;
        }

        t.grid[r][c] = Cell::Digit(d);
        let mut got = 0;
        if !t.row_is_ok(r) {
            got += 1;
        }
        if !t.col_is_ok(c) {
            got += 1;
        }
        if !t.group_is_ok(r / 3, c / 3) {
            got += 1;
        }

        t.grid[r][c] = old;
        got - had
    }

    fn probability(delta: i16, temperature: f32) -> f32 {
        if delta <= 0 {
            1.1
        } else {
            let power: f32 = (-delta as f32) / temperature;
            power.exp()
        }
    }

    pub fn solve_simulated_annealing_single(mut t: Table) -> Result<Table, &'static str> {
        let emptys = t.empty_cells();

        let mut values = generate_digits(emptys.len());
        apply(&mut t, &emptys, &values);
        let mut opt: i16 = inversions(&t);

        let mut temperature: f32 = 1.0;
        let mut iters = 0;

        while opt != 0 && iters < ANNEALING_ITERS {
            let i = thread_rng().gen_range(0..emptys.len());
            let d = thread_rng().gen_range(1..=9);

            let delta = calc_change(&mut t, emptys[i].0, emptys[i].1, d);

            if thread_rng().gen_range(0.0..1.0) < probability(delta, temperature) {
                t.grid[emptys[i].0][emptys[i].1] = Cell::Digit(d);
                values[i] = d;
                opt += delta;
            }

            temperature *= 0.99;
            iters += 1;
        }
        if t.solved() {
            Ok(t)
        } else {
            Err("No solution found")
        }
    }
}

pub mod app {
    use crate::grid::Table;

    pub fn table_from_stdin() -> Table {
        let t;
        loop {
            match Table::read() {
                Ok(tb) => {
                    t = tb;
                    break;
                }
                Err(e) => println!("Error: {}.\n Try again.", e),
            }
        }
        t
    }
}
