extern crate termion;

use termion::{color, style};

use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeSet;

struct Floor {
    temp: String,
    floor: Vec<usize>,
    width: usize,
    depth: usize,
    area: usize,
    combined: BTreeSet<usize>

}

impl Floor {
    fn new() -> Floor {
        Floor{
            temp: String::from(""),
            floor: Vec::new(),
            width: 0,
            depth: 0,
            area: 0,
            combined: BTreeSet::new()
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
        let mut result: Vec<usize> = Vec::new();
        if pos >= self.width {
            result.push(pos - self.width);
        }
        if pos < self.area - self.width {
            result.push(pos + self.width);
        }
        let x: usize = pos % self.width;
        if x > 0 {
            result.push(pos - 1);
        }
        if x < self.width - 1 {
            result.push(pos + 1);
        }
        result
    }

    fn lowest(&self, pos: usize) -> Option<usize> {
        let d = self.floor[pos];
        for neighbour in self.adjacent(pos) {
            if self.floor[neighbour] <= d {
                return None;
            }
        }
        return Some(d);
    }

    fn highest(&self, pos: usize) -> bool {
        let d = self.floor[pos];
        let neighbours = self.adjacent(pos);
        for neighbour in &neighbours {
            if self.floor[*neighbour] == d {
                return true;
            }
        }
        for neighbour in &neighbours {
            if self.floor[*neighbour] >= d {
                return false;
            }
        }
        return true;
    }

    fn flow(&self, pos: usize, included: &BTreeSet<usize>) -> Option<Vec<usize>> {
        let d = self.floor[pos];
        let mut result: Vec<usize> = Vec::new();
        for neighbour in self.adjacent(pos) {
            if included.contains(&neighbour) {
                continue;
            }
            if self.floor[neighbour] < d {
                return None;
            }
            result.push(neighbour);
        }
        Some(result)
    }

    fn basin(&mut self, pos: usize) -> usize {
        let mut included: BTreeSet<usize> = BTreeSet::new();
        let mut pending: Vec<Vec<usize>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        pending[self.floor[pos]] = vec![pos];
        for i in self.floor[pos]..9 {
            let level = &pending[i].clone();
            for point in level {
                if let Some(neighbours) = self.flow(*point, &included) {
                    included.insert(*point);
                    self.combined.insert(*point);
                    for neighbour in neighbours {
                        pending[self.floor[neighbour]].push(neighbour);
                    }
                }
            }
        }

        let area: usize = included.len();
        area
    }
}

impl fmt::Display for Floor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for i in (0..self.area).step_by(self.width as usize) {
            for j in i..i+self.width {
                if self.combined.contains(&j) {
                    fmt.write_fmt(format_args!("{}{}{}", style::Bold, self.floor[j], style::Reset))?;
                } else if self.floor[j] == 9 {
                    fmt.write_fmt(format_args!("{}{}{}", color::Fg(color::Blue), self.floor[j], color::Fg(color::Reset)))?;
                } else if self.highest(j) {
                    fmt.write_fmt(format_args!("{}{}{}", color::Fg(color::Green), self.floor[j], color::Fg(color::Reset)))?;
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
        let mut areas: Vec<usize> = vec![0, 0, 0];
        for pos in 0..floor.area {
            if floor.lowest(pos).is_some() {
                let area = floor.basin(pos);
                // println!("{}", floor);

                if area > areas[0] {
                    areas[2] = areas[1];
                    areas[1] = areas[0];
                    areas[0] = area;
                } else if area > areas[1] {
                    areas[2] = areas[1];
                    areas[1] = area;
                } else if area > areas[2] {
                    areas[2] = area;
                }
                // println!("Area: {}, areas: {:?}", area, areas);
            }
        }
        println!("Product = {}", areas[0] * areas[1] * areas[2]);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
