use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn orbit_count(orbits: &HashMap<String, Vec<String>>, object: &str, current: u32) -> u32 {
    current + match orbits.get(object) {
        Some(orbiting_objects) => orbiting_objects.iter().map(|x| orbit_count(orbits, x, current + 1)).sum(),
        None => 0
    }
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut orbits = HashMap::new();
    
    for line in reader.lines() {
        let line = line.unwrap();
        let objects: Vec<_> = line.split(')').collect();
        let a = objects[0].to_string();
        let b = objects[1].to_string();
        orbits.entry(a).or_insert(vec!()).push(b);
    }
    
    println!("{}", orbit_count(&orbits, "COM", 0));
}