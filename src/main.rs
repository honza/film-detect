use film_detect;
use std::{io, path};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    for arg in &args[1..] {
        let fujifilm_settings = film_detect::get_fujifilm_settings(path::Path::new(&arg))?;
        println!("{:?}", fujifilm_settings);
    }

    Ok(())
}
