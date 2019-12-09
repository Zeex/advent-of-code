use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn find_object(orbit_map: &HashMap<String, Vec<String>>, target: &String, root: &String, distance: u32) -> u32 {
    return match orbit_map.get(root) {
        Some(objects) =>
            if objects.contains(target) {
                distance
            } else {
                objects.iter().map(|x| find_object(orbit_map, target, x, distance + 1)).sum()
            },
        None => 0
    };
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut orbit_map = HashMap::new();
    let mut reverse_orbit_map = HashMap::new();
    
    for line in reader.lines() {
        let line = line.unwrap();
        let objects: Vec<_> = line.split(')').collect();
        orbit_map.entry(objects[0].to_string())
            .or_insert(vec!())
            .push(objects[1].to_string());
        reverse_orbit_map.insert(objects[1].to_string(), objects[0].to_string());
    }
    
    let mut transfers = 0_u32;
    let mut search_root = &reverse_orbit_map["YOU"];
    loop {
        let forward_transfers = find_object(&orbit_map, &"SAN".to_string(), &search_root, 0);
        if forward_transfers > 0 {
            transfers += forward_transfers;
            break;
        }
        match reverse_orbit_map.get(search_root) {
            Some(object) => {
                search_root = object;
                transfers += 1;
            },
            None => break
        }
    }
    
    println!("{}", transfers);
}