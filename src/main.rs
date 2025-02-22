use std::{env, fs::{rename, File}, io::{BufReader, Read}, path::{Path, PathBuf}};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use num_format::{Locale, ToFormattedString};
use infer;

const BUFFER_SIZE: usize = 40;

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

    let mut file_paths: Vec<String> = vec![];

    for file in path.read_dir().expect("reading directory failed") {
        match file {
            Err(err) => eprintln!("Could not read file: {}", err),
            Ok(file) => {
                if !file.file_type().unwrap().is_file() { continue; }
                file_paths.push(file.path().to_string_lossy().to_string());
            }
        }
    }

    let mut files_changed = 0;
    let mut files_failed = 0;

    println!("Reading directory {} : {} files", path.to_str().unwrap(), file_paths.len().to_formatted_string(&Locale::de));

    let multibar = MultiProgress::new();
    let bar_style = ProgressStyle::with_template("\n {elapsed} | ETA: {eta} | {wide_bar} | {pos}/{len} ").unwrap();
    let bar = multibar.add(ProgressBar::new(file_paths.len().try_into().unwrap()));
    bar.set_style(bar_style.clone());

    bar.reset();

    for file_path in file_paths {
        let mut path = PathBuf::from(&file_path);
        
        let file_handle = File::open(&file_path).expect("failed opening file");
        let mut reader = BufReader::with_capacity(BUFFER_SIZE,  file_handle);

        let mut buffer = [0u8; BUFFER_SIZE];
        reader.read(&mut buffer).expect(&format!("reading of first {} bytes of file failed", BUFFER_SIZE));

        if !(infer::is_video(&buffer) || infer::is_image(&buffer)) { continue; }

        let file_infer = infer::get(&buffer);
        if file_infer.is_none() { 
            multibar.println(format!(" {:12} | {} -> Unable to detect", "?".repeat(12), &file_path)).unwrap();

            files_failed += 1;
            bar.inc(1);
            continue;
        }

        let expected_extension = file_infer.unwrap().extension();
        path.set_extension(expected_extension);

        if file_path == path.to_str().unwrap() {
            bar.inc(1);
            continue;
        }
        
        rename(&file_path, &path).expect("failed renaming file");
        multibar.println(format!(" {:12} | {} -> {}", file_infer.unwrap().mime_type(), &file_path, &path.to_str().unwrap())).unwrap();
        files_changed += 1;
        bar.inc(1);
    }

    bar.finish();

    println!("\nChanged : {}\nFailed  : {}", files_changed.to_formatted_string(&Locale::de), files_failed.to_formatted_string(&Locale::de));
}
