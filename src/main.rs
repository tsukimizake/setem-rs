use std::collections::HashSet;
use std::path::Path;
use setem_rs::{process_elm_file, process_elm_directory, generate_setters, setup_parser};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <elm_file_or_directory> [prefix]", args[0]);
        std::process::exit(1);
    }
    
    let path = &args[1];
    let prefix = args.get(2).map(|s| s.as_str()).unwrap_or("s_");
    
    let mut parser = setup_parser();
    
    let mut all_identifiers = HashSet::new();
    
    if Path::new(path).is_file() {
        process_elm_file(path, &mut parser, &mut all_identifiers);
    } else if Path::new(path).is_dir() {
        process_elm_directory(path, &mut parser, &mut all_identifiers);
    } else {
        eprintln!("Error: {} is not a valid file or directory", path);
        std::process::exit(1);
    }
    
    let setters = generate_setters(&all_identifiers, prefix);
    println!("{}", setters);
}

