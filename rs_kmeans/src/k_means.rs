type Point = Vec<f64>;

fn mean(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

fn distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b)
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[derive(Debug)]
pub struct KMeansResult {
    pub centroids: Vec<Point>,
    pub iteration: u32,
    pub error: Option<f64>,
    pub did_reach_steady_state: bool,
}

pub struct KMeans {
    k: usize,
    data: Vec<Point>,
    error: Option<f64>,
    iterations: u32,
    centroids: Vec<Point>,
    centroid_assignments: Vec<usize>,
    iteration_logs: Vec<KMeansResult>,
}

impl KMeans {
    pub fn new(k: usize, data: Vec<Point>) -> KMeans {
        KMeans {
            k,
            data,
            error: None,
            iterations: 0,
            centroids: vec![],
            centroid_assignments: vec![],
            iteration_logs: vec![],
        }
    }

    pub fn print_logs(&self) {
        for log in self.iteration_logs.iter() {
            println!("{:#?}", log);
        }
    }

    fn reset(&mut self) {
        self.error = None;
        self.iterations = 0;
        self.centroids = self.init_random_centroids();
        self.centroid_assignments.clear();
        self.iteration_logs.clear();
    }

    fn get_dimensionality(&self) -> usize {
        self.data[0].len()
    }

    fn get_range_for_dimension(&self, n: usize) -> (f64, f64) {
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for point in &self.data {
            let value = point[n];
            if value < min {
                min = value;
            }
            if value > max {
                max = value;
            }
        }
        (min, max)
    }

    fn get_all_dimension_ranges(&self) -> Vec<(f64, f64)> {
        let dimensionality = self.get_dimensionality();
        let mut dimension_ranges = Vec::with_capacity(dimensionality);
        for dimension in 0..dimensionality {
            dimension_ranges.push(self.get_range_for_dimension(dimension));
        }
        dimension_ranges
    }

    fn init_random_centroids(&mut self) -> Vec<Vec<f64>> {
        let mut centroids = Vec::new();
        let dimensionality = self.get_dimensionality();
        let dimension_ranges = self.get_all_dimension_ranges();
        for _ in 0..self.k {
            let mut point = Vec::with_capacity(dimensionality);
            for (min, max) in dimension_ranges.iter() {
                let value = min + rand::random::<f64>() * (max - min);
                point.push(value);
            }
            centroids.push(point);
        }
        centroids
    }

    fn assign_point_to_centroid(&mut self, point_index: usize) -> bool {
        let last_assigned_centroid = self.centroid_assignments[point_index];
        let point = &self.data[point_index];
        let mut min_distance = f64::MAX;
        let mut assigned_centroid = 0;
        for (i, centroid) in self.centroids.iter().enumerate() {
            let distance_to_centroid = distance(point, centroid);
            if distance_to_centroid < min_distance {
                min_distance = distance_to_centroid;
                assigned_centroid = i;
            }
        }
        self.centroid_assignments[point_index] = assigned_centroid;
        assigned_centroid != last_assigned_centroid
    }

    fn assign_points_to_centroids(&mut self) -> bool {
        let mut changed = false;
        for point_index in 0..self.data.len() {
            if self.assign_point_to_centroid(point_index) {
                changed = true;
            }
        }
        changed
    }

    fn get_points_for_centroid(&self, centroid_index: usize) -> Vec<Point> {
        let mut points = Vec::new();
        for (i, &assigned_centroid) in self.centroid_assignments.iter().enumerate() {
            if assigned_centroid == centroid_index {
                points.push(self.data[i].clone());
            }
        }
        points
    }

    fn update_centroid_location(&mut self, centroid_index: usize) {
        let points = self.get_points_for_centroid(centroid_index);
        let dimensionality = self.get_dimensionality();
        let mut new_centroid = vec![0.0; dimensionality];
        for dimension in 0..dimensionality {
            let dimension_values: Vec<f64> = points.iter().map(|point| point[dimension]).collect();
            new_centroid[dimension] = mean(&dimension_values);
        }
        self.centroids[centroid_index] = new_centroid;
    }

    fn update_centroid_locations(&mut self) {
        for centroid_index in 0..self.k {
            self.update_centroid_location(centroid_index);
        }
    }

    fn calculate_error(&mut self) -> f64 {
        let mut sum_distance_squared = 0.0;
        for point_index in 0..self.data.len() {
            let assigned_centroid = self.centroid_assignments[point_index];
            let point = &self.data[point_index];
            let centroid = &self.centroids[assigned_centroid];
            let this_distance = distance(point, centroid);
            sum_distance_squared += this_distance * this_distance;
        }
        (sum_distance_squared / self.data.len() as f64).sqrt()
    }

    pub fn solve(&mut self, max_iterations: u32) -> KMeansResult {
        self.reset();
        self.centroid_assignments = vec![0; self.data.len()];

        let mut did_reach_steady_state = false;
        while self.iterations < max_iterations && !did_reach_steady_state {
            let assignments_changed = self.assign_points_to_centroids();
            self.update_centroid_locations();
            let new_error = self.calculate_error();

            self.iteration_logs.push(KMeansResult {
                centroids: self.centroids.clone(),
                iteration: self.iterations,
                error: Some(new_error),
                did_reach_steady_state: !assignments_changed,
            });

            if new_error == self.error.unwrap_or(f64::INFINITY) {
                did_reach_steady_state = true;
            }

            self.error = Some(new_error);

            if !assignments_changed {
                break;
            }

            self.iterations += 1;
        }

        KMeansResult {
            centroids: self.centroids.clone(),
            iteration: self.iterations,
            error: self.error,
            did_reach_steady_state,
        }
    }
}
