use std::{
    fs, io, path::Path
};
use clap::ArgMatches;


pub fn parse_path(args: &ArgMatches, callback: fn(&Path) -> io::Result<()>) -> io::Result<()> {
    let paths = match args.get_many::<String>("path") {
        Some(p) => p.map(|s| Path::new(s.as_str())).collect::<Vec<&Path>>(),
        None => vec![Path::new(".")],
    };

    for path in paths {
        if !path.exists() {
            println!("Path \"{}\" doesn't exist!", path.display());
            continue;
        }

        if path.is_dir() {
            read_dir(path, args.get_flag("recursive"), callback)?;
        } else if path.is_file() {
            callback(path)?;
        }
    }

    Ok(())
}

fn read_dir(dir: &Path, recursive: bool, callback: fn(&Path) -> io::Result<()>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && recursive {
            read_dir(&path, recursive, callback)?;
        } else if path.is_file() {
            callback(&path)?;
        }
    }

    Ok(())
}