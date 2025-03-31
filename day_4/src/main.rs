mod helpers;
use std::io::BufRead;

fn create_grid() -> Vec<Vec<char>> {
    // Get the cli arg for the filepath
    let file_path_arg = helpers::get_filepath_arg();
    
    // Get a file buffer from the arg
    let file_buffer = helpers::get_file(file_path_arg);
    let file_content: Vec<_> = file_buffer.lines().collect();
    let grid_size = file_content.len();
    let row = vec!['.'; grid_size];
    let mut grid = vec![row; grid_size];
    for index in 0..grid_size {
        let row = match &file_content[index] {
            Ok(row) => row,
            Err(e) => {
                panic!("Cannot parse word row!: {}",e)
            }
        };
        grid[index] = row.chars().collect();
        
    }
    grid
}

fn main() {
    
    println!("Running total: {:?}", grid);
}
