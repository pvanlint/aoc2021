use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BTreeMap;

struct Tree {
    children: Vec<String>
}

impl Tree {
    fn new() -> Self {
        Self {
            children: Vec::new()
        }
    }

    fn add_child(&mut self, child: &str) {
        self.children.push(String::from(child));
    }
}

struct Waypoint {
    path: String,
    id: String
}

impl Waypoint {
    fn cycle(&self) -> bool {
        if self.id != self.id.to_lowercase() {
            return false;
        }
        for i in self.path.split("-") {
            if self.id == i {
                return true;
            }
        }
        false
    }
}

struct Graph {
    index: BTreeMap<String, Tree>
}

impl Graph {
    fn new() -> Graph {
        Graph {
            index: BTreeMap::new()
        }
    }

    fn add(&mut self, link: &str) {
        let (from, end) = link.split_at(link.find("-").unwrap());
        let to: &str = &end[1..];
        self.add_node(from);
        self.add_node(to);
        self.connect(from, to);
        self.connect(to, from);
    }

    fn add_node(&mut self, id: &str) {
        if !self.index.contains_key(id) {
            self.index.insert(String::from(id), Tree::new());
        }
    }

    fn connect(&mut self, from: &str, to: &str) {

        if let Some(parent) = self.index.get_mut(from) {
            parent.add_child(to);
        }
    }

    fn dfs_visit(&self, f: impl Fn(&str)) -> usize{
        let mut stack: Vec<Waypoint> = Vec::new();
        let mut count: usize = 0;

        stack.push(Waypoint { path: String::from(""), id: String::from("start")});

        while stack.len() > 0 {
            if let Some(w) = stack.pop() {
                let new_path = String::from(&w.path) + "-" + &w.id;
                if w.id == "end" {
                    (f)(&new_path);
                    count += 1;
                    continue;
                }
                if w.cycle() {
                    println!("Hit cycle {},{}", w.path, w.id);
                    continue;
                }
                if let Some(t) = self.index.get(&w.id) {
                    for child in &t.children {
                        stack.push(Waypoint { path: String::from(&new_path), id: String::from(child) });
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("No parameters specified");
        return;
    }

    let mut graph = Graph::new();
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(strval) = line {
                println!("Parsing {}", strval);
                graph.add(&strval);
            }
        }
    }
    let count = graph.dfs_visit(|s| println!("Item: {}", s));
    println!("Count: {}", count)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
