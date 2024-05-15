// film detect
// Copyright (C) 2024 Honza Pokorny <honza@pokorny.ca>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
