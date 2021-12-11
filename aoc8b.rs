use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeMap;

struct Frequency {
    freq: BTreeMap<char, u32>
}

impl Frequency {
    fn new() -> Frequency {
        Frequency{
            freq: BTreeMap::new()
        }
    }

    fn add(&mut self, value: &str) {
        for c in value.chars() {
            if let Some(f) = self.freq.get_mut(&c) {
                *f += 1;
            } else {
                self.freq.insert(c, 1);
            }
        }
    }

    fn extract_char(&self, frequency: u32) -> Option<char> {
        for (k,v) in &self.freq {
            if v == &frequency {
                return Some(*k);
            }
        }
        None
    }
}

struct Length {
    length: BTreeMap<usize, Vec<String>>
}

impl Length {
    fn new() -> Length {
        Length {
            length: BTreeMap::new()
        }
    }

    fn add(&mut self, value: &str) {
        let len = value.len();
        if let Some(l) = self.length.get_mut(&len) {
            (*l).push(String::from(value));
        } else {
            self.length.insert(len, vec![String::from(value)]);
        }
    }

    fn extract_char(&self, mapping:&BTreeMap<char, char>, len: usize) -> Option<char> {
        if let Some(strs) = self.length.get(&len) {
            for str in strs {
                let result:Vec<char> = str.chars().filter(|x| !mapping.contains_key(&x)).collect();
                if result.len() == 1 {
                    return Some(result[0]);
                }
            }
        }
        None
    }
}

struct Parser {
    freq: Frequency,
    length: Length,
    mapping: BTreeMap<char, char>
}

impl Parser {
    fn new() -> Parser {
        Parser {
            freq: Frequency::new(),
            length: Length::new(),
            mapping: BTreeMap::new()
        }
    }

    fn add(&mut self, value: &str) {
        self.freq.add(value);
        self.length.add(value);
    }

    fn extract(&mut self) {
        if let Some(b) = self.freq.extract_char(6) {
            self.mapping.insert(b, 'b');
        }
        if let Some(e) = self.freq.extract_char(4) {
            self.mapping.insert(e, 'e');
        }
        if let Some(f) = self.freq.extract_char(9) {
            self.mapping.insert(f, 'f');
        }
        if let Some(c) = self.length.extract_char(&self.mapping, 2) {
            self.mapping.insert(c, 'c');
        }
        if let Some(a) = self.length.extract_char(&self.mapping, 3) {
            self.mapping.insert(a, 'a');
        }
        if let Some(d) = self.length.extract_char(&self.mapping, 4) {
            self.mapping.insert(d, 'd');
        }
        if let Some(g) = self.length.extract_char(&self.mapping, 7) {
            self.mapping.insert(g, 'g');
        }
    }
}

struct Numbers {
    numbers: BTreeMap<&'static str, &'static str>
}

impl Numbers {
    fn new() -> Numbers {
        let mut numbers:BTreeMap<&'static str, &'static str> = BTreeMap::new();
        numbers.insert("abcefg", "0");
        numbers.insert("cf", "1");
        numbers.insert("acdeg", "2");
        numbers.insert("acdfg", "3");
        numbers.insert("bcdf", "4");
        numbers.insert("abdfg", "5");
        numbers.insert("abdefg", "6");
        numbers.insert("acf", "7");
        numbers.insert("abcdefg", "8");
        numbers.insert("abcdfg", "9");

        Numbers {
            numbers: numbers
        }
    }

    fn convert_num(&self, mapping:&BTreeMap<char, char>, outputs:&Vec<&str>) -> usize {
        let mut num = String::from("");
        for o in outputs {
            let mut result:Vec<&char> = o.chars().map(|c| &mapping[&c]).collect();
            result.sort();
            let new_o:String = result.into_iter().collect();
            if let Some(n) = self.numbers.get(new_o.as_str()) {
                num.push_str(n);
            }
        }
        return num.parse().unwrap();
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
        let numbers = Numbers::new();

        let mut total = 0;
        for line in lines {
            if let Ok(strval) = line {
                let mut parser = Parser::new();

                let entries: Vec<&str> = strval.split(' ').collect();
                let mut input:bool = true;
                let mut outputs:Vec<&str> = Vec::new();
                for value in entries {
                    if value == "|" {
                        input = false;
                    } else if input {
                        parser.add(value);
                    } else {
                        outputs.push(value);
                    }
                }
                parser.extract();
                total += numbers.convert_num(&parser.mapping, &outputs);
            }
        }
        println!("Total: {}", total);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
