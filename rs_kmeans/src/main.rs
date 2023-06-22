use data::EXAMPLE_2D3K;

use k_means::KMeans;

use crate::{
    data::{EXAMPLE_2DNK, EXAMPLE_3D3K},
    solver::KMeansAutoSolver,
};

mod data;
mod k_means;
mod solver;

fn main() {
    println!("Solution for 2d data with 3 clusters:");
    println!("-------------------------------------");
    let data: Vec<_> = EXAMPLE_2D3K.into_iter().map(|row| row.to_vec()).collect();
    let mut ex_1_solver = KMeans::new(3, data);
    let ex_1_centroids = ex_1_solver.solve(10000);
    println!("{:?}\n", ex_1_centroids);

    println!("Iteration log for 2d data with 3 clusters:");
    println!("------------------------------------------");
    ex_1_solver.print_logs();

    println!("Test 2d data with 3 clusters five times:");
    println!("-----------------------------------------");
    for _ in 0..5 {
        let solution = ex_1_solver.solve(1000);
        println!("{:?}", solution);
    }

    println!("\n-----------------------------------------\n");

    println!("Solution for 3d data with 3 clusters:");
    println!("-------------------------------------");
    let data: Vec<_> = EXAMPLE_3D3K.into_iter().map(|row| row.to_vec()).collect();
    let mut ex_2_solver = KMeans::new(3, data);
    let ex_2_centroids = ex_2_solver.solve(10000);
    println!("{:?}\n", ex_2_centroids);

    println!("Iteration log for 3d data with 3 clusters:");
    println!("------------------------------------------");
    ex_2_solver.print_logs();

    println!("Test 3d data with 3 clusters five times:");
    println!("-----------------------------------------");
    for _ in 0..5 {
        let solution = ex_2_solver.solve(1000);
        println!("{:?}", solution);
    }

    println!("\n-----------------------------------------\n");

    println!("Solving example: 2d data with unknown clusters:");
    println!("===============================================\n");
    let data: Vec<_> = EXAMPLE_2DNK.into_iter().map(|row| row.to_vec()).collect();
    let mut ex_3_solver = KMeansAutoSolver::new(1, 5, 5, data);
    let ex_3_solution = ex_3_solver.solve(10000);
    println!("{:?}", ex_3_solution.unwrap());
}
