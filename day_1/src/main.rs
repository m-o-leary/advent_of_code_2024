use std::env;
use std::process;
use std::io::{self, BufReader, BufRead};
use std::fs::File;

fn main() -> io::Result<()> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if a file path is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }
    
    // Get the file path from arguments
    let file_path = &args[1];
    
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
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader.lines().collect();
    let mut left_number_list: Vec<i32> = vec![0; lines.len()];
    let mut right_number_list: Vec<i32> = vec![0; lines.len()];
    let split_str = "   ";
    for line_index in 0..lines.len() {
        let line_result = match &lines[line_index] {
            Ok(line) => line,
            Err(e) => {
                panic!("{}", e)
            }
        };
        let fields: Vec<&str> = line_result.split(split_str).collect();
        // print!("{:?}\n", fields);

        let left_number: i32 = fields[0].parse().unwrap();
        let right_number: i32 = fields[1].parse().unwrap();
        
        left_number_list[line_index] = left_number;
        right_number_list[line_index] = right_number;
    }

    left_number_list.sort();
    right_number_list.sort();
    let mut total_diff = 0;
    for line_index in 0..lines.len() {
        // Calculate difference between 2 lists
        let diff = left_number_list[line_index].abs_diff(right_number_list[line_index]);
        total_diff = total_diff + diff;
    }

    println!("Total difference: {}", total_diff);
    
    Ok(())
}