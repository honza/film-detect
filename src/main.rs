use film_detect::FilmError;
use serde_json;
use std::path;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Filename to operate on
    file: String,

    /// Output JSON
    #[arg(long)]
    json: bool,

    // Sets a film simulation directory
    #[arg(short, long, value_name = "DIR")]
    simulations: Option<path::PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    match film_detect::get_fujifilm_settings(path::Path::new(&cli.file)) {
        Ok(fujifilm_settings) => {
            if cli.json {
                println!("{}", serde_json::json!(fujifilm_settings));
            } else {
                println!("{}", fujifilm_settings);
            }
        }
        Err(e) => {
            let message = match e {
                FilmError::NotAFujifilmFile => format!("Error: not a Fujifilm file"),
                FilmError::IO(io_err) => format!("I/O error: {}", io_err),
                FilmError::Exif(exif_error) => format!("Exif parsing error: {}", exif_error),
                FilmError::UnexpectedValue(value) => {
                    format!("Found unexpected value while parsing: {}", value)
                }
            };
            println!("{}", message);
            std::process::exit(0);
        }
    }
}
