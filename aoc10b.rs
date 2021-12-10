use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeMap;

struct Chunks {
    close: BTreeMap<char, char>,
    score: BTreeMap<char, usize>
}

impl Chunks {
    fn new() -> Chunks {
        let mut close: BTreeMap<char, char> = BTreeMap::new();
        let mut score: BTreeMap<char, usize> = BTreeMap::new();
        close.insert('(', ')');
        close.insert('[', ']');
        close.insert('{', '}');
        close.insert('<', '>');
        score.insert('(', 1);
        score.insert('[', 2);
        score.insert('{', 3);
        score.insert('<', 4);

        Chunks {
            close: close,
            score: score
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

    fn score(&self, ch: char, score: usize) -> usize {
        if let Some(s) = self.score.get(&ch) {
            return score * 5 + *s;
        }
        0
    }

    fn calculate(&self, str: &str) -> usize {
        let mut v: Vec<char> = str.chars().collect();
        let mut idx: usize = 0;
        let mut score: usize = 0;
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
                return 0;
            }
        }
        v.reverse();
        let _str: String = v.iter()
                           .filter(|c| *c != &' ')
                           .inspect(|&c| score = self.score(*c, score))
                           .collect();
        score
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
        let mut scores: Vec<usize> = Vec::new();
        for line in lines {
            if let Ok(strval) = line {
                let score: usize = chunks.calculate(&strval);
                if score > 0 {
//                    println!("Score: {}", score);
                    scores.push(score);
                }
            }
        }
        scores.sort();

//        println!("Scores: {:?}", scores);
        println!("Result: {}", scores[(scores.len() - 1) / 2]);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
