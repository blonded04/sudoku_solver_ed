use std::time::Instant;
use sudoku_solver_ed::app;
use sudoku_solver_ed::solver::solve_simulated_annealing_single;

const MAX_ATTEMPTS: u8 = 65;

fn main() {
    println!("Write sudoku puzzle to solve:");
    let t = app::table_from_stdin();

    let start = Instant::now();
    for i in 0..MAX_ATTEMPTS {
        match solve_simulated_annealing_single(t.clone()) {
            Ok(t_res) => {
                let spent_time = start.elapsed().as_secs_f32();
                println!("Solution found in {} seconds:", spent_time);
                t_res.print();
                break;
            }
            Err(_e) => {
                println!(
                    "Annealing didn't find solution, attempts left: {}.",
                    MAX_ATTEMPTS - i - 1
                );
            }
        }
    }
}
