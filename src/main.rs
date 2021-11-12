mod cli;
mod level;
mod level_detail;
mod model;
mod wavefunction;

use cli::Cli;
use level::Level;
use level_detail::LevelDetail;
use model::Model;
use wavefunction::WaveFunction;

use rayon::prelude::*;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let level = Level::from_ogmo(&args.input)?;

    println!("Started... \n");
    
    (1..=args.count).collect::<Vec<_>>().par_iter().for_each(|idx| {
        mutate_level(*idx, args.clone(), level.clone());
    });
    
    println!("\nFinished !");

    Ok(())
}

fn mutate_level(idx: u8, args: Cli, mut level: Level) {
    level.layers.par_iter_mut().enumerate().for_each(|(layer_idx, layer)| {
        let size = if let Some(dim) = &args.dimensions {
            if dim.len() == 1 {
                (dim[0], dim[0])
            }else if dim.len() == 2 {
                (dim[0], dim[1])
            }else {
                panic!("Dimensions should be atleast; [w, h] or [n]");
            }
        }else {
            (layer.w, layer.h)
        };
        
        loop {
            let mut model = Model::new(size.clone(), layer);
            let new_layer = model.run();

            if !new_layer.is_none() {
                let mut new_layer = new_layer.unwrap();
                new_layer.idx = layer.idx;
                *layer = new_layer;
                break;
            }else {
                println!("x Level {}, Layer {}, failed. Trying again...", idx, layer_idx + 1);
            }
        }
        
        println!("* Level {}, Layer {}, done.", idx, layer_idx + 1);
    });

    println!(">>> Level {} completed.", idx);

    level.save_ogmo(idx, &args).expect("Could not save the level.");
}