
use std::env;
use std::process;
use std::io::BufReader;
use std::fs::File;

pub fn get_file(file_path: String) -> BufReader<File> {
    // Create a buffered reader
    let f = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            // fallback in case of failure.
            // you could log the error, panic, or do anything else.
            panic!("{}", e)
        }
    };
    // Collect all lines into a vector
    BufReader::new(f)
}

pub fn get_filepath_arg() -> String {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if a file path is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }
    // Get the file path from arguments
    let file_path = &args[1];
    return file_path.to_owned();
}
