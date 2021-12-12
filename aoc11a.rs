extern crate termion;

use termion::{color, style};

use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Floor {
    temp: String,
    floor: Vec<usize>,
    width: usize,
    depth: usize,
    area: usize,
    flashes: usize

}

impl Floor {
    fn new() -> Floor {
        Floor{
            temp: String::from(""),
            floor: Vec::new(),
            width: 0,
            depth: 0,
            area: 0,
            flashes: 0
        }
    }

    fn add(&mut self, str: &str) {
        if self.width == 0 {
            self.width = str.len();
        }
        self.depth += 1;
        self.temp.push_str(str);
    }

    fn convert(&mut self) {
        const RADIX: u32 = 10;
        if self.temp != "" {
            self.area = self.temp.len();
            self.floor = self.temp.chars().map(|c| c.to_digit(RADIX).unwrap() as usize).collect();
            self.temp.clear();
        }

    }

    // Returns the vector of adjacent points
    fn adjacent(&self, pos: usize) -> Vec<usize> {
        let x: usize = pos % self.width;
        let mut result: Vec<usize> = Vec::new();
        if pos >= self.width {
            result.push(pos - self.width);
            if x > 0 {
                result.push(pos - self.width - 1);
            }
            if x < self.width - 1 {
                result.push(pos - self.width + 1);
            }
        }
        if pos < self.area - self.width {
            result.push(pos + self.width);
            if x > 0 {
                result.push(pos + self.width - 1);
            }
            if x < self.width - 1 {
                result.push(pos + self.width + 1);
            }
        }
        if x > 0 {
            result.push(pos - 1);
        }
        if x < self.width - 1 {
            result.push(pos + 1);
        }
        result
    }

    fn energise(&mut self) {
        self.floor.iter_mut().for_each(|x| *x += 1);
    }

    fn flash(&mut self) -> bool {
        let mut flashes: usize = 0;
        let mut f: Vec<usize> = Vec::new();
        for i in (0..self.area).step_by(self.width as usize) {
            for j in i..i+self.width {
                if self.floor[j] > 9 {
                    f.push(j);
                    flashes += 1;
                    self.floor[j] = 0;
                    for a in self.adjacent(j) {
                        if self.floor[a] > 0 {
                            self.floor[a] += 1;
                        }
                    }
                }
            }
        }
        self.flashes += flashes;
        return flashes > 0;
    }
}

impl fmt::Display for Floor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("{} = {} * {}\n", self.area, self.width, self.depth))?;
        for i in (0..self.area).step_by(self.width as usize) {
            fmt.write_fmt(format_args!("{:03}: ", i))?;
            for j in i..i+self.width {
                if self.floor[j] == 0 {
                    fmt.write_fmt(format_args!("{}{}{}", style::Bold, self.floor[j], style::Reset))?;
                } else if self.floor[j] > 9 {
                    fmt.write_fmt(format_args!("{}*{}", color::Fg(color::Yellow), color::Fg(color::Reset)))?;
                } else {
                    fmt.write_fmt(format_args!("{}{}{}", color::Fg(color::Yellow), self.floor[j], color::Fg(color::Reset)))?;
                }
            }
            fmt.write_str("\n")?;
        }
        Ok(())
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
        floor.convert();
        for i in 0..100 {
            floor.energise();
            while floor.flash() {}
            println!("Step {}:\n{}", i, floor);
        }
        println!("Flashes = {}", floor.flashes);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
