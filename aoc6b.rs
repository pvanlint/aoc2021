use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Model {
    fish: Vec<u64>
}

impl Model {
    fn new(starting_ages: String) -> Model {
        let mut fish: Vec<u64> = vec![0; 9];
        let ages: Vec<usize> = starting_ages.
            split(",").
            map(|x| x.parse::<usize>()).
            filter(|x| x.is_ok()).
            map(|x| x.unwrap()).
            collect();

        for age in ages {
            fish[age] += 1;
        }

        Model{
            fish: fish
        }
    }

    fn next_day(&mut self) {
        let zero_day = self.fish[0];
        // Shift all values into the lower bucket
        for i in 1..=8 {
            self.fish[i-1] = self.fish[i];
        }
        self.fish[8] = zero_day;
        self.fish[6] += zero_day;
    }

    fn get_total(&self) -> u64 {
        let total:u64 = self.fish.iter().sum();
        total
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("No parameters specified");
        return;
    }

    let days = 256;
    println!("Reading {:?}", &args[1]);
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(strval) = line {
                let mut model = Model::new(strval);
                for _i in 0..days {
                    model.next_day();
                }
                println!("Total after {} days: {}", days, model.get_total());
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
