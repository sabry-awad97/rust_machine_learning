use std::collections::HashMap;

type VoteCount = (String, usize);
type DistanceMapEntry = (usize, f64, String);

fn distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b)
        .map(|(&a_point, &b_point)| (b_point - a_point).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub struct Knn {
    k: usize,
    data: Vec<Vec<f64>>,
    labels: Vec<String>,
}

impl Knn {
    pub fn new(k: usize, data: Vec<Vec<f64>>, labels: Vec<String>) -> Self {
        assert_eq!(data.len(), labels.len());
        Self { k, data, labels }
    }

    fn generate_distance_map(&self, point: &[f64]) -> Vec<DistanceMapEntry> {
        let mut map: Vec<DistanceMapEntry> = Vec::new();

        let mut max_distance_in_map: Option<f64> = None;
        for (index, other_point) in self.data.iter().enumerate() {
            let other_point_label = &self.labels[index];
            let this_distance = distance(point, other_point);

            if max_distance_in_map.is_none() || this_distance < max_distance_in_map.unwrap() {
                map.push((index, this_distance, other_point_label.to_string()));

                map.sort_by(|(_, dist_a, _), (_, dist_b, _)| dist_a.partial_cmp(dist_b).unwrap());

                if map.len() > self.k {
                    map.pop();
                }

                max_distance_in_map = Some(map.last().unwrap().1);
            }
        }

        map
    }

    pub fn predict(
        &self,
        point: &[f64],
    ) -> Option<(String, Vec<VoteCount>, Vec<DistanceMapEntry>)> {
        let map = self.generate_distance_map(point);
        let mut vote_counts: HashMap<String, usize> = HashMap::new();
        for (_, _, label) in map.iter().take(self.k) {
            *vote_counts.entry(label.to_string()).or_insert(0) += 1;
        }

        let mut sorted_votes: Vec<_> = vote_counts.into_iter().collect();
        sorted_votes.sort_by(|(_, count_a), (_, count_b)| count_b.cmp(count_a));

        if let Some((label, _)) = sorted_votes.first().cloned() {
            let votes = map.iter().take(self.k).cloned().collect::<Vec<_>>();
            Some((label, sorted_votes, votes))
        } else {
            None
        }
    }
}
