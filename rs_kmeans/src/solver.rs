use crate::k_means::{KMeans, Point};

#[derive(Debug, Clone)]
pub struct Solution {
    pub centroids: Vec<Point>,
    pub iteration: u32,
    pub error: Option<f64>,
    pub did_reach_steady_state: bool,
    pub k: usize,
    pub current_trial: usize,
}

impl Solution {
    fn new(
        centroids: Vec<Point>,
        iteration: u32,
        error: Option<f64>,
        did_reach_steady_state: bool,
        k: usize,
        current_trial: usize,
    ) -> Solution {
        Solution {
            centroids,
            iteration,
            error,
            did_reach_steady_state,
            k,
            current_trial,
        }
    }
}

pub struct KMeansAutoSolver {
    best: Option<Solution>,
    log: Vec<Solution>,
    k_min: usize,
    k_max: usize,
    max_trials: usize,
    data: Vec<Point>,
}

impl KMeansAutoSolver {
    pub fn new(
        k_min: usize,
        k_max: usize,
        max_trials: usize,
        data: Vec<Point>,
    ) -> KMeansAutoSolver {
        KMeansAutoSolver {
            best: None,
            log: vec![],
            k_min,
            k_max,
            max_trials,
            data,
        }
    }

    fn reset(&mut self) {
        self.best = None;
        self.log.clear();
    }

    pub fn solve(&mut self, max_iterations: u32) -> Option<Solution> {
        self.reset();

        for k in self.k_min..self.k_max {
            for current_trial in 0..self.max_trials {
                let mut solver = KMeans::new(k, self.data.clone());
                let result = solver.solve(max_iterations);
                let solution = Solution::new(
                    result.centroids,
                    result.iteration,
                    result.error,
                    result.did_reach_steady_state,
                    k,
                    current_trial,
                );
                self.log.push(solution.clone());
                if self.best.is_none()
                    || solution.error.unwrap() < self.best.as_ref().unwrap().error.unwrap()
                {
                    self.best = Some(solution.clone());
                }
            }
        }

        self.best.clone()
    }
}
