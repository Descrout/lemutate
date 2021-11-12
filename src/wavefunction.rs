use std::collections::{HashMap, HashSet};
use crate::level::Layer;

pub struct WaveFunction {
    coefficients: Vec<Vec<HashSet<i16>>>,
    weights: HashMap<i16, i16>
}


impl WaveFunction{
    pub fn new(size: (usize, usize), weights: HashMap<i16, i16>) -> Self {
        let coefficients = WaveFunction::init_coefficients(size, &weights);
        Self{coefficients, weights}
    }

    pub fn init_coefficients(size: (usize, usize), weights: &HashMap<i16, i16>) -> Vec<Vec<HashSet<i16>>> {
        let mut coefficients = Vec::with_capacity(size.1 as usize);

        for j in 0..size.1 {
            coefficients.push(Vec::with_capacity(size.0));
            for _ in 0..size.0 {
                coefficients[j].push(weights.keys().cloned().collect());
            }
        }

        coefficients
    }

    pub fn get(&self, pos: (usize, usize)) -> HashSet<i16> {
        self.coefficients[pos.0][pos.1].clone()
    }

    pub fn shannon_entropy(&self, j: usize, i: usize) -> f32 {
        let mut sum_of_weights: f32 = 0.0;
        let mut sum_of_log_weights: f32 = 0.0;

        for opt in &self.coefficients[j][i] {
            let weights = *self.weights.get(opt).unwrap() as f32;
            sum_of_weights += weights;
            sum_of_log_weights += weights * weights.ln();
        }

        sum_of_weights.ln() - (sum_of_log_weights / sum_of_weights)
    }

    pub fn collapse(&mut self, pos: (usize, usize)) {
        let j = pos.0;
        let i = pos.1;
        let options = &self.coefficients[j][i];

        let mut valid_weights = self.weights.clone();
        valid_weights.retain(|&k, _| options.contains(&k));

        let total_weights = valid_weights.values().sum::<i16>() as f32;
        let mut rnd = fastrand::f32() * total_weights;

        for (tile, weight) in valid_weights.into_iter() {
            rnd -= weight as f32;
            if rnd < 0.0 {
                self.coefficients[j][i].retain(|&k| k == tile);
                break;
            }
        }
    }

    pub fn constrain(&mut self, pos: (usize, usize), forbidden_tile: &i16) {
        self.coefficients[pos.0][pos.1].remove(forbidden_tile);
    }

    pub fn get_collapsed(&self, pos: (usize, usize)) -> Option<i16> {
        let options = self.get(pos);

        assert!(options.len() <= 1);

        let mut tile: Option<i16> = None;
        for k in options.iter() {
            tile = Some(*k);
            break;
        }
        tile
    }

    pub fn get_all_collapsed(&self) -> Option<Layer> {
        let w = self.coefficients[0].len();
        let h = self.coefficients.len();
        let mut layer = Layer::new(w, h, 0);

        for j in 0..h {
            for i in 0..w {
                let tile = self.get_collapsed((j, i));
                if let Some(t) = tile {
                    layer.data[j][i] = t;
                }else {
                    return None;
                }
            }
        }

        Some(layer)
    }

    pub fn is_fully_collapsed(&self) -> bool {
        for row in self.coefficients.iter() {
            for tiles in row.iter() {
                if tiles.len() > 1 {
                    return false
                }
            }
        }
        true
    }
}