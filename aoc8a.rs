use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("No parameters specified");
        return;
    }

    println!("Reading {:?}", &args[1]);
    if let Ok(lines) = read_lines(&args[1]) {
        let mut unique = 0;
        for line in lines {
            if let Ok(strval) = line {
                let entries: Vec<&str> = strval.split(' ').collect();
                for value in &entries[11..] {
                    unique += match value.len() {
                        2|3|4|7=>1,
                        _=>0
                    };
                }
            }
        }
        println!("Unique: {}", unique);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
