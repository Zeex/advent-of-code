use std::fs::File;
use std::io::{BufRead, BufReader};

fn calc_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel < 0 {
        return 0;
    } else {
        return fuel + calc_fuel(fuel);   
    }
}

fn main() {
    let f = File::open("input_2.txt");
    let reader = BufReader::new(f.unwrap());
    let mut total_fuel = 0;
    
    for (_, line) in reader.lines().enumerate() {
        let module_mass = line.unwrap().parse::<i32>().unwrap();
        total_fuel += calc_fuel(module_mass);
    }
    
    println!("Total fuel needed: {}", total_fuel);
}
