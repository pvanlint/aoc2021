use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeMap;

fn fuel(mut distance: u32) -> u32 {
    let mut total = 0;
    while distance > 0 {
        total += distance;
        distance -= 1;
    }
    return total;
}

struct Model {
    crabs: BTreeMap<u32, u32>,
    min: u32,
    max: u32

}

impl Model {
    fn new(starting_pos: &String) -> Model {
        let mut crabs: BTreeMap<u32, u32> = BTreeMap::new();
        let positions: Vec<u32> = starting_pos.
            split(",").
            map(|x| x.parse::<u32>()).
            filter(|x| x.is_ok()).
            map(|x| x.unwrap()).
            collect();

        let mut min = positions[0];
        let mut max = positions[0];

        for pos in positions {
            if pos > max {
                max = pos;
            }
            if pos < min {
                min = pos;
            }
            if let Some(x) = crabs.get_mut(&pos) {
                *x += 1;
            } else {
                crabs.insert(pos, 1);
            }
        }
        Model{
            crabs: crabs,
            min: min,
            max: max
        }
    }

    fn fuel_moving_right(&self, target: u32) -> u32{
        return self.crabs.iter().filter(|x| x.0 < &target).map(|x| x.1 * fuel(target - x.0)).sum();
    }

    fn fuel_moving_left(&self, target: u32) -> u32{
        return self.crabs.iter().filter(|x| x.0 > &target).map(|x| x.1 * fuel(x.0 - target)).sum();
    }
}

struct Cost {
    pos: u32,
    left: u32,
    right: u32
}

impl Cost {
    fn new(model: &Model, pos: u32) -> Cost {
        let left = model.fuel_moving_left(pos);
        let right = model.fuel_moving_right(pos);
        Cost {
            pos: pos,
            left: left,
            right: right
        }
    }

    fn fuel(&self) -> u32 {
        return self.left + self.right;
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
        for line in lines {
            if let Ok(strval) = line {
                let model = Model::new(&strval);
                let mut last = Cost::new(&model, model.min);
                for i in (model.min+1)..model.max {
                    let cost = Cost::new(&model, i);
                    if cost.fuel() > last.fuel() {
                        break;
                    }
                    last = cost;
                }
                println!("Pos: {}, Cost: {}", last.pos, last.fuel());
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
