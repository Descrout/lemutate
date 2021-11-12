use crate::{LevelDetail, WaveFunction, level::Layer};

pub struct Model {
    size: (usize, usize),
    level_detail: LevelDetail,
    wavefunction: WaveFunction,
}

impl Model {
    pub fn new(size: (usize, usize), layer: &Layer) -> Self {
        let details = LevelDetail::new_with_weights(layer);
        let wavefunction = WaveFunction::new(size, details.1);
        Self{size, level_detail: details.0, wavefunction}
    }

    pub fn run(&mut self) -> Option<Layer> {

        while !self.wavefunction.is_fully_collapsed() {
            self.iterate();
        }

        self.wavefunction.get_all_collapsed()
    }

    pub fn iterate(&mut self) {
        let pos = self.min_entropy_pos();
        self.wavefunction.collapse(pos);
        self.propagate(pos);
    }

    pub fn min_entropy_pos(&self) -> (usize, usize) {
        let mut pos: Option<(usize, usize)> = None;
        let mut min_entropy: Option<f32> = None;

        for j in 0..self.size.1 {
            for i in 0..self.size.0 {
                if self.wavefunction.get((j as usize, i as usize)).len() == 1 {
                    continue;
                }

                let entropy = self.wavefunction.shannon_entropy(j as usize, i as usize);
                let entropy_with_noise = entropy - (fastrand::f32() / 1000.0);

                if min_entropy.is_none() || entropy_with_noise < min_entropy.unwrap() {
                    min_entropy = Some(entropy_with_noise);
                    pos = Some((j as usize, i as usize));
                }
            }
        }

        pos.unwrap()
    }

    pub fn propagate(&mut self, pos: (usize, usize)) {
        let mut stack = vec![pos];

        while stack.len() > 0 {
            let cur_pos = stack.pop().unwrap();
            let cur_possible_tiles = self.wavefunction.get(cur_pos);
            let valid_dirs = LevelDetail::valid_dirs(cur_pos.1, cur_pos.0, self.size.0 as usize, self.size.1 as usize);

            for dir in valid_dirs.into_iter() {
                let other_pos = ((cur_pos.0 as i8 + dir.1) as usize, (cur_pos.1 as i8 + dir.0) as usize);
                let other_possible_tiles = self.wavefunction.get(other_pos);

                for other_tile in other_possible_tiles.iter() {
                    let mut any_possible = false;

                    for cur_tile in cur_possible_tiles.iter() {
                        if self.level_detail.check(*cur_tile, *other_tile, dir.clone()) {
                            any_possible = true;
                            break;
                        }
                    }

                    if !any_possible {
                        self.wavefunction.constrain(other_pos, other_tile);
                        stack.push(other_pos);
                    }
                }
            }
        }
    }
}