mod helpers;
use regex::Regex;
use std::io::BufRead;

fn parse_calcs(calcs: &String, can_mul: &mut bool) -> i32 {
    let re = Regex::new(r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)|(?<do>do\(\))|(?<dont>don\'t\(\))").unwrap();
    let mut results: Vec<i32> = vec![];
    println!("Can multiply: {}", can_mul);
    for cap in re.captures_iter(calcs) {
        let total_match = &cap[0];
        // print!("Match: {} - ", total_match);
        if total_match.contains("mul") {
            let left = match &cap["left"].parse::<i32>() {
                Ok(number) => *number,
                Err(_) => {panic!("Cannot parse '{}'", &cap["left"])}
            };
            let right = match &cap["right"].parse::<i32>() {
                Ok(number) => *number,
                Err(_) => {panic!("Cannot parse '{}'", &cap["right"])}
            };
            // println!("{}*{} - use? : {}",left, right, can_mul);
            
            if *can_mul {
                results.push(left * right);
            }
        } else if total_match.contains("don't()") {
            // println!("switching to no multiplication");
            *can_mul = false;
        } else if total_match.contains("do()"){
            // println!("turning back on multiplication");
            *can_mul = true;
        }
    }
    return results.iter().sum();
}

fn main() {
    // Get the cli arg for the filepath
    let file_path_arg = helpers::get_filepath_arg();

    // Get a file buffer from the arg
    let file_buffer = helpers::get_file(file_path_arg);

    // Total 
    let mut total = 0;

    // Read the lines
    let instructions: Vec<_> = file_buffer.lines().collect();
    let mut can_mul = true;
    for maybe_report in instructions {
        let report = match maybe_report {
            Ok(report) => report,
            Err(e) => {
                panic!("{}",e)
            }
        };
        total += parse_calcs(&report, &mut can_mul);
        println!("Running total: {}", total);
            
    }
    println!("Sum: {:?}", total);
}
