use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Floor {
    floor: String,
    width: usize,
    depth: usize

}

impl Floor {
    fn new() -> Floor {
        Floor{
            floor: String::from(""),
            width: 0,
            depth: 0
        }
    }

    fn add(&mut self, str: &str) {
        if self.width == 0 {
            self.width = str.len();
        }
        self.depth += 1;
        self.floor.push_str(str);
    }

    fn lowest(&self, x: usize, y: usize) -> Option<usize> {
        let pos = y * self.width + x;
        let ch = &self.floor[pos..pos+1];
        if x > 0 && &self.floor[pos - 1..pos] <= ch {
            return None;
        }
        if x < self.width - 1 && &self.floor[pos + 1..pos + 2] <= ch {
            return None;
        }
        if y > 0 && &self.floor[pos - self.width..pos - self.width + 1] <= ch {
            return None;
        }
        if y < self.depth - 1 && &self.floor[pos + self.width..pos + self.width + 1] <= ch {
            return None;
        }
        return Some(ch.parse().unwrap());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("No parameters specified");
        return;
    }

    println!("Reading {:?}", &args[1]);
    if let Ok(lines) = read_lines(&args[1]) {
        let mut floor = Floor::new();
        for line in lines {
            if let Ok(strval) = line {
                floor.add(&strval);
            }
        }
        let mut risk = 0;
        for x in 0..floor.width {
            for y in 0..floor.depth {
                if let Some(r) = floor.lowest(x, y) {
                    risk += r + 1;
                }
            }
        }
        println!("Risk: {}", risk);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
