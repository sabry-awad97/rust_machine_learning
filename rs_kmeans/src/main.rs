use data::EXAMPLE_2D3K;

use k_means::KMeans;

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
    println!()
}
