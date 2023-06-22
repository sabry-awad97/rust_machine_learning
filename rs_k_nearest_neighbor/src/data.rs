use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Point {
    label: String,
    point: [f64; 2],
}

pub fn get_points() -> (Vec<String>, Vec<Vec<f64>>) {
    let data: Vec<Point> = serde_json::from_str(include_str!("../data.json")).unwrap();
    let labels = data.iter().map(|p| p.label.clone()).collect();
    let points = data.iter().map(|p| p.point.to_vec()).collect();
    (labels, points)
}
