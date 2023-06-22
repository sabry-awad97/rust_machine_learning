use data::EXAMPLE_2D3K;

use k_means::KMeans;

mod data;
mod k_means;
mod solver;

fn main() {
    println!("Solution for 2d data with 3 clusters:");
    println!("-------------------------------------");
    let data: Vec<_> = EXAMPLE_2D3K.into_iter().map(|row| row.to_vec()).collect();
    let mut k_means = KMeans::new(3, data);
    let ex_1_centroids = k_means.solve(10000);
    println!("{:#?}", ex_1_centroids);

    println!("Iteration log for 2d data with 3 clusters:");
    println!("------------------------------------------");
    k_means.print_logs();
}
