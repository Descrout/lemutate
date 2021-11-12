use serde_json::Value;
use crate::cli::Cli;

#[derive(Debug, Clone)]
pub struct Layer {
    pub w: usize,
    pub h: usize,
    pub idx: usize,
    pub data: Vec<Vec<i16>>,
}

impl Layer {
    pub fn new(w: usize, h: usize, idx: usize) -> Self {
        Self{
            w,
            h,
            idx,
            data: vec![vec![0; w];h]
        }
    }
}

#[derive(Debug, Clone)]
pub struct Level {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Layer>,
    pub level_data: Value,
}


impl Level {
    pub fn from_ogmo(path: &std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>>  {
        let level_str = std::fs::read_to_string(path)?;
        let level_data: Value = serde_json::from_str(&level_str)?;

        let width = level_data["width"].as_u64().unwrap() as u32;
        let height = level_data["height"].as_u64().unwrap() as u32;

        let mut layers = Vec::<Layer>::new();

        for layer_data in level_data["layers"].as_array().unwrap() {
            let data_2d = layer_data["data2D"].as_array();

            if data_2d.is_none() {
                continue
            }

            let mut layer = Layer::new(
                layer_data["gridCellsX"].as_u64().unwrap() as usize,
                layer_data["gridCellsY"].as_u64().unwrap() as usize,
                layers.len()
            );

            for (j, row) in data_2d.unwrap().iter().enumerate() {
                for (i, num) in row.as_array().unwrap().iter().enumerate() {
                    layer.data[j][i] = num.as_i64().unwrap() as i16;
                }
            }

            layers.push(layer);
        }

        Ok(Level{width, height, layers, level_data})
    }

    pub fn save_ogmo(&mut self, idx: u8, args: &Cli) -> Result<(), Box<dyn std::error::Error>> {
        if args.input.is_dir() {
            return Err("Input must not be a directory.".into());
        }

        let idx = idx.to_string();
        let old_name = args.input.file_stem().unwrap().to_str().unwrap();

        let mut filename = str::replace(&args.name, "%", &idx);
        filename = str::replace(&filename, "$", &old_name);

        let mut folder = str::replace(&args.folder, "%", &idx);
        folder = str::replace(&folder, "$", &old_name);


        let mut path = args.input.parent().unwrap().to_path_buf();
        path.push(folder);
        
        if !path.exists() {
            std::fs::create_dir(&path)?;
        }

        path.push(filename);

        let layers = self.level_data["layers"].as_array_mut().unwrap();

        let mut tile_w: i64 = 0;
        let mut tile_h: i64 = 0;

        for layer in self.layers.iter() {
            layers[layer.idx]["gridCellsX"] = serde_json::json!(layer.w);
            layers[layer.idx]["gridCellsY"] = serde_json::json!(layer.h);
            layers[layer.idx]["data2D"] = serde_json::json!(layer.data);

            let tw = layers[layer.idx]["gridCellWidth"].as_i64().unwrap() * layer.w as i64;
            let th = layers[layer.idx]["gridCellHeight"].as_i64().unwrap() * layer.h as i64;
            if tw > tile_w {
                tile_w = tw;
            } 
            if th > tile_h {
                tile_h = th;
            }
        }

        self.level_data["width"] = serde_json::json!(tile_w);
        self.level_data["height"] = serde_json::json!(tile_h);

        serde_json::to_writer(&std::fs::File::create(path)?, &self.level_data)?;
        Ok(())
    }
}