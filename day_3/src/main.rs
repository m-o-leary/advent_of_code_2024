mod helpers;
use regex::Regex;
use std::io::BufRead;

fn parse_calcs(calcs: &String) -> i32 {
    let re = Regex::new(r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"don\'t\(\).*do\(\)").unwrap();
    let mut matches: Vec<(i32, i32)> = vec![];
    let dont_bounds: Vec<(usize,usize)> = do_re.find_iter(calcs).map(|m| (m.start(), m.end())).collect();

    let group_names: Vec<&str> =
        re.capture_names().skip(1).map(|x| x.unwrap()).collect();
    
    let mut cum_sum: i32 = 0;
    
    'match_loop: for caps in re.captures_iter(calcs) {
        let mut left: i32 = 0; 
        let mut right: i32 = 0;

        for name in &group_names {
            if let Some(m) = caps.name(name) {
                'inner: for (start, stop) in &dont_bounds {
                    let match_start = m.start();
                    if match_start > *start && match_start < *stop {
                        continue 'match_loop;
                    } 
                    if match_start < *start {
                        break 'inner;
                    }
                }
                if name == &"left" {
                    left = match m.as_str().parse::<i32>() {
                        Ok(number) => i32::from(number),
                        Err(e) => {
                            panic!("{}", e);
                        }
                    };
                } else {
                    right = match m.as_str().parse::<i32>() {
                        Ok(number) => i32::from(number),
                        Err(e) => {
                            panic!("{}", e);
                        }
                    };

                }

            }
        }
        matches.push((left,right));
        cum_sum += left * right;
    }

    println!("{:?}", matches.len());
    cum_sum
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
    for maybe_report in instructions {
        let report = match maybe_report {
            Ok(report) => report,
            Err(e) => {
                panic!("{}",e)
            }
        };
        total += parse_calcs(&report);
            
    }
    println!("Sum: {:?}", total);
}
