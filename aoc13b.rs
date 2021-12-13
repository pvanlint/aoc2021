extern crate termion;

use termion::{color, style};

use std::env;
use std::cmp;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Sheet {
    cell: Vec<usize>,
    width: usize,
    height: usize,
    area: usize
}

impl Sheet {
    fn new(width: usize, height: usize) -> Sheet {
        let area = width * height;
        Sheet {
            cell: vec![0; area],
            width,
            height,
            area
        }
    }

    fn add(&mut self, x: usize, y: usize) {
        let pos = y * self.width + x;
        self.cell[pos] = 1;
    }

    fn add_row(output: &mut Vec<usize>, output_offset: usize, input: &Vec<usize>, input_offset: usize, length: usize) {
        for col in 0..length {
            output[output_offset + col] += input[input_offset + col];
        }
    }

    fn add_fold_row(output: &mut Vec<usize>, output_offset: usize, input: &Vec<usize>, input_offset: usize, length: usize, crease: usize) {
        let mid = (length - 1) / 2;
        if crease >= mid {
            for col in 0..crease {
                output[output_offset + col] += input[input_offset + col];
            }
            let mut dest = crease;
            for col in (crease + 1)..length {
                dest -= 1;
                output[output_offset + dest] += input[input_offset + col];
            }
        } else if crease < mid {
            let mut dest = length - 1 - crease - crease;
            for col in 0..crease {
                output[output_offset + dest] += input[input_offset + col];
                dest += 1;
            }
            for col in (crease + 1)..length {
                dest -= 1;
                output[output_offset + dest] += input[input_offset + col];
            }
        }
    }

    fn fold_y(&mut self, crease: usize) {
        let top = crease;
        let bottom = self.height - 1 - crease;
        let new_height = cmp::max(top, bottom);
        let new_area = self.width * new_height;
        let mut new_cell: Vec<usize> = vec![0; new_area];

        let mid = (self.height - 1) / 2;
        if crease >= mid {
            let mut base = 0;
            for row in 0..crease {
                Sheet::add_row(&mut new_cell, base, &self.cell, base, self.width);
                base += self.width;
            }
            let mut fold = base;
            for row in (crease + 1)..self.height {
                base -= self.width;
                fold += self.width;
                Sheet::add_row(&mut new_cell, base, &self.cell, fold, self.width);
            }
        } else {
            let mut base = 0;
            let mut fold = self.area;
            for row in 0..crease {
                fold -= self.width;
                Sheet::add_row(&mut new_cell, base, &self.cell, fold, self.width);
                base += self.width;
            }
            for row in (crease + 1)..self.height {
                base -= self.width;
                Sheet::add_row(&mut new_cell, base, &self.cell, base, self.width);
            }
        }

        self.cell = new_cell;
        self.height = new_height;
        self.area = new_area;
    }

    fn fold_x(&mut self, crease: usize) {
        let left = crease;
        let right = self.height - 1 - crease;
        let new_width = cmp::max(left, right);
        let new_area = new_width * self.height;
        let mut new_cell: Vec<usize> = vec![0; new_area];

        let mut base: usize = 0;
        let mut fold: usize = 0;
        for row in 0..self.height {
            Sheet::add_fold_row(&mut new_cell, base, &self.cell, fold, self.width, crease);
            base += new_width;
            fold += self.width;
        }
        self.cell = new_cell;
        self.width = new_width;
        self.area = new_area;
    }

    fn count(&self) -> usize {
        return self.cell.iter().map(|x| if x > &0 {1} else {0}).sum();
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("{} = {} * {}\n", self.area, self.width, self.height))?;
        for i in (0..self.area).step_by(self.width as usize) {
            fmt.write_fmt(format_args!("{:03}: ", i))?;
            for j in i..i+self.width {
                if self.cell[j] > 0 {
                    fmt.write_fmt(format_args!("{}#{}", style::Bold, style::Reset))?;
                } else {
                    fmt.write_str(".");
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
    let mut width: usize = 0;
    let mut height: usize = 0;
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(strval) = line {
                if strval == "" {
                    break;
                }
                let tokens: Vec<usize> = strval.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
                let x = tokens[0];
                let y = tokens[1];
                if x >= width {
                    width = x + 1;
                }
                if y >= height {
                    height = y + 1;
                }
            }
        }
    }
    println!("Width: {}, Height: {}", width, height);
    println!("Reading {:?}", &args[1]);
    if let Ok(lines) = read_lines(&args[1]) {
        let mut sheet = Sheet::new(width, height);
        for line in lines {
            if let Ok(strval) = line {
                if strval.starts_with("fold along x=") {
                    let x: usize = strval[13..].parse::<usize>().unwrap();
                    sheet.fold_x(x);
//                    println!("x fold: {}", x);
                } else if strval.starts_with("fold along y=") {
                    let y: usize = strval[13..].parse::<usize>().unwrap();
                    sheet.fold_y(y);
//                    println!("y fold: {}", y);
                } else if strval != "" {
                    let tokens: Vec<usize> = strval.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
                    let x = tokens[0];
                    let y = tokens[1];
                    sheet.add(x, y);
                }
            }
        }
        println!("Sheet:\n{}", sheet);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
