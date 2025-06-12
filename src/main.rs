mod package;
mod payload;

use package::Package;
use payload::Payload;

use glob::glob;
use clap::Parser;

use std::{
    error::Error, fs::remove_dir_all, path::PathBuf
};

use log::{error, info, LevelFilter};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input `.pkg`
    input: String,

    /// Output directory
    output: Option<String>,
    
    /// Overwrite existing files.
    /// Immediately removes the output directory; be careful
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    overwrite: bool,
}

fn main() {
    pretty_env_logger::formatted_builder().filter_level(LevelFilter::Debug).init();
    if let Err(e) = _main() {
        error!("{e:}");
    }
}

fn _main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    let input = PathBuf::from(args.input);
    if input.is_dir() { 
        return Err("Expected a `.pkg`, not a directory".into());
    }

    let output = match args.output {
        Some(out) => PathBuf::from(out),
        None => {
            let stem = input.file_stem().unwrap();
            PathBuf::from(stem)
        }
    };
    
    if output.exists() {
        if args.overwrite {
            remove_dir_all(&output)?;
        } else {
            return Err("--overwrite was not specified, but the directory exists".into());
        }
    }
    
    info!("Extracting `.pkg` root");
    
    Package::new(&input)
        .unpack_into(&output)?;
    
    for entry in glob("**/Payload").unwrap() {
        info!("Extracting payload");
        let path = entry?;
        Payload::new(&path)
            .unpack_into(&path.parent().ok_or("&path.parent() returned `None`")?)?;
    }
    
    Ok(())
}
