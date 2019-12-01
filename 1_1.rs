use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input_1.txt");
    let reader = BufReader::new(f.unwrap());
    let mut total_fuel = 0;
    
    for (_, line) in reader.lines().enumerate() {
        let mass = line.unwrap().parse::<u32>().unwrap();
        let fuel = mass / 3 - 2;
        total_fuel += fuel;
    }
    
    println!("Total fuel needed: {}", total_fuel)
}