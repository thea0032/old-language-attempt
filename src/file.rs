use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::{dict::storage::Data, format2, input::*, parse};
use crate::dict::*;
pub fn read_new_file() -> Vec<String>{
    loop{
        let mut filename = get_str("Enter a filename to compile (will add extension):");
        filename.push_str(".meta");
        if let Ok(lines) = read_lines(filename.clone()) {
            let mut result: Vec<String> = vec![filename];
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    result.push(ip);
                }
            }
            return result;
        }
        println!("Couldn't read file {}! Please enter a valid filename.", filename.clone());
    }
}
pub fn read_file(path:String)->Vec<String>{
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(path) {
        let mut result: Vec<String> = vec![];
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                result.push(ip);
            }
        }
        return result;
    }
    panic!("Couldn't read file!");
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn process_new_file(path:String, dict:&mut Dict) -> Data{
    let file = read_file(path.clone());
    let handled = format2::handle(file);
    dict.new_file();
    dict.files.new_file(path);
    let result = parse::parse_grid(handled, dict);
    dict.files.close_curr_file();
    dict.drop_scope();
    return result;
}