use std::time::Instant;
use sudoku_solver_ed::app;
use sudoku_solver_ed::solver::solve_dfs_single;

fn main() {
    println!("Write sudoku puzzle to solve:");
    let t = app::table_from_stdin();

    let start = Instant::now();
    match solve_dfs_single(t.clone()) {
        Ok(t_res) => {
            let spent_time = start.elapsed().as_secs_f32();
            println!("Solution found in {} seconds:", spent_time);
            t_res.print();
        }
        Err(_e) => {
            println!("Sudoku has no solution.");
        }
    }
}
