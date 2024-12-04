use std::env;
use std::process;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn get_file(file_path: String) -> BufReader<File> {
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

fn get_filepath_arg() -> String {
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

fn check_levels(levels: &Vec<i32>) -> bool {
    let mut is_only_dec: bool = true;
    let mut is_only_inc: bool = true;
    for index in 0..levels.len() {
        let left = match levels.get(index) {
            Some(number) => *number,
            None => i32::from(1_0000)
        };
        let right = match levels.get(index+1) {
            Some(number) => *number,
            None => {
                break;
            }
        };
        if left < right {
            is_only_dec = false;
        }
        if left > right {
            is_only_inc = false;
        }
        let delta = &(left-right).abs();
        let delta_between_bounds = (1..=3).contains(delta);
        if !((is_only_dec | is_only_inc) & delta_between_bounds) {
            // println!("Left: {}, Right: {} - Unsafe", left, right);
            return false;

        }
        
    }
    return true;
}

fn is_safe(levels: &Vec<i32>) -> bool {
    println!("Original: {:?}", levels);
    if !check_levels(levels) {
        for level in 0..levels.len() {
            // Create a copy of levels without the current index
            let mut levels_minus_index = levels.clone();
            levels_minus_index.remove(level);
            let safe = check_levels(&levels_minus_index); 
            println!("Modified: {:?}, Safe: {}", levels_minus_index, safe);
            if safe { return true; }
        }
        return false;
    }

    return true;

    
}

fn main() {
    // Get the cli arg for the filepath
    let file_path_arg = get_filepath_arg();

    // Get a file buffer from the arg
    let file_buffer = get_file(file_path_arg);
    
    // Read the lines
    let reports: Vec<_> = file_buffer.lines().collect();
    let mut safe_reports = 0;
    for maybe_report in reports {
        let report = match maybe_report {
            Ok(report) => report,
            Err(e) => {
                panic!("{}",e)
            }
        };
        
        let levels: Vec<i32> = report.split(" ").map(|level| {
            let level: i32 = match level.parse::<i32>() {
                Ok(number) => i32::from(number),
                Err(e) => {
                    panic!("{}", e);
                }
            };
            level
        }).collect();

        let is_safe = is_safe(&levels);
        safe_reports += if is_safe { 1 } else { 0 };
        println!("{:?} -  is safe: {}", levels, is_safe);
    }

    println!("Safe reports: {}", safe_reports)

    
}
