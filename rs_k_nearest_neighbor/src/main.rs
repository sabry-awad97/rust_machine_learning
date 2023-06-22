use crate::{data::get_points, knn::Knn};

mod data;
mod knn;
fn main() {
    println!("Testing height and weight with k=5");
    println!("==========================");

    let (labels, data) = get_points();
    let solver = Knn::new(5, data, labels);

    println!("Testing a 'definitely male' point:");
    println!("{:#?}", solver.predict(&[200.0, 75.0]).unwrap());

    println!("\nTesting a 'probably male' point:");
    println!("{:#?}", solver.predict(&[170.0, 70.0]).unwrap());

    println!("\nTesting a 'totally uncertain' point:");
    println!("{:#?}", solver.predict(&[140.0, 64.0]).unwrap());

    println!("\nTesting a 'probably female' point:");
    println!("{:#?}", solver.predict(&[130.0, 63.0]).unwrap());

    println!("\nTesting a 'definitely female' point:");
    println!("{:#?}", solver.predict(&[120.0, 60.0]).unwrap());
}
