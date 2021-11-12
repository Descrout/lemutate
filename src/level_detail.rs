use std::collections::{HashMap, HashSet};
use crate::level::Layer;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Dir(pub i8, pub i8);

#[derive(Debug)]
pub struct LevelDetail {
    compatibilities: HashSet<(i16, i16, Dir)>,
}

impl LevelDetail {
    pub fn new_with_weights(layer: &Layer) -> (Self, HashMap<i16, i16>) {
        let mut compatibilities = HashSet::new();
        let mut weights = HashMap::new();
        
        for (j, row) in layer.data.iter().enumerate() {
            for (i, cur_tile) in row.iter().enumerate() {
                let weight = weights.entry(*cur_tile).or_insert(0);
                *weight += 1;

                let valid_dirs = Self::valid_dirs(i, j, layer.w, layer.h);
                
                for dir in valid_dirs.into_iter() {
                    let other_tile = layer.data[(j as i8 + dir.1) as usize][(i as i8 + dir.0) as usize];
                    compatibilities.insert((*cur_tile, other_tile, dir));
                }
            }
        }

        (Self {compatibilities}, weights)
    }

    pub fn valid_dirs(i: usize, j: usize, w: usize, h: usize) -> Vec<Dir> {
        let mut dirs = Vec::new();

        if i > 0 {
            dirs.push(Dir(-1, 0));

            if j > 0 {
                dirs.push(Dir(-1, -1));
            } 
            if j < h - 1 {
                dirs.push(Dir(-1, 1));
            }
        }
        if i < w - 1 {
            dirs.push(Dir(1, 0));

            if j > 0 {
                dirs.push(Dir(1, -1));
            } 
            if j < h - 1 {
                dirs.push(Dir(1, 1));
            }
        }

        if j > 0 {
            dirs.push(Dir(0, -1));
        }
        if j < h - 1 {
            dirs.push(Dir(0, 1));
        }

        dirs
    }

    pub fn check(&self, tile1: i16, tile2: i16, dir: Dir) -> bool {
        self.compatibilities.contains(&(tile1, tile2, dir))
    }
}