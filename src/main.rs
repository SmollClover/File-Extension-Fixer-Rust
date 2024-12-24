use std::{env, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let folder = args.get(1);

    if folder.is_none() {
        eprintln!("Please provide a folder path as an argument.");
        return;
    }

    let path = Path::new(folder.unwrap());

    if !path.is_dir() {
        eprintln!("Please provide a folder path as an argument.");
        return;
    }

    // print!("Reading directory {} : ", "tmp");

    for file in path.read_dir().expect("Reading directory failed") {
        match file {
            Err(err) => eprintln!("Could not read file: {}", err),
            Ok(file) => {
                dbg!(file);
            }
        }
    }
}
