use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeMap;

struct Chunks {
    close: BTreeMap<char, char>,
    penalty: BTreeMap<char, usize>
}

impl Chunks {
    fn new() -> Chunks {
        let mut close: BTreeMap<char, char> = BTreeMap::new();
        let mut penalty: BTreeMap<char, usize> = BTreeMap::new();
        close.insert('(', ')');
        close.insert('[', ']');
        close.insert('{', '}');
        close.insert('<', '>');
        penalty.insert(')', 3);
        penalty.insert(']', 57);
        penalty.insert('}', 1197);
        penalty.insert('>', 25137);

        Chunks {
            close: close,
            penalty: penalty
        }
    }

    fn expected_match(&self, ch: char) -> Option<char> {
        if let Some(result) = self.close.get(&ch) {
            return Some(*result);
        }
        None
    }

    fn is_open(&self, ch: char) -> bool {
        return self.close.contains_key(&ch);
    }

    fn validate(&self, str: &str) -> usize {
        let mut v: Vec<char> = str.chars().collect();
        let mut idx: usize = 0;
        // Check non open char
        for i in 1..v.len() {
            if self.is_open(v[i]) {
                idx = i;
            } else if Some(v[i]) == self.expected_match(v[idx]) {
                // Replace matching braces with spaces
                v[i] = ' ';
                v[idx] = ' ';
                // Move idx back to non-space character or back to start
                while idx > 0 && v[idx] == ' ' {
                    idx -= 1;
                }
            } else {
                if let Some(penalty) = self.penalty.get(&v[i]) {
                    return *penalty;
                }
                println!("Error, but unknown penalty for '{}'", v[i]);
            }
        }
        0
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
        let chunks = Chunks::new();
        let mut penalty = 0;
        for line in lines {
            if let Ok(strval) = line {
                let p = chunks.validate(&strval);
//                println!("Penalty: {}", p);
                penalty += p;
            }
        }
        println!("Overall penalty: {}", penalty);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
