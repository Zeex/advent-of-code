use std::cmp::{min, max};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_wire_coords(s: String) -> Vec<(i32, i32)> {
    let path = s.trim()
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let mut coords: Vec<(i32, i32)> = vec!((0, 0));
    let mut x = 0;
    let mut y = 0;
    for p in path {
        let n = p[1..].to_string().parse::<i32>().unwrap();
        match &p[..1] {
            "U" => y += n,
            "D" => y -= n,
            "L" => x -= n,
            "R" => x += n,
            _ => continue
        }
        coords.push((x, y));
    }
    return coords;
}

fn find_intersections(wire1: &Vec<(i32, i32)>, wire2: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut points = vec!();
    
    for i in 1..wire1.len() {
        let (x11, y11) = wire1[i - 1];
        let (x12, y12) = wire1[i];
        
        for j in 1..wire2.len() {
            let (x21, y21) = wire2[j - 1];
            let (x22, y22) = wire2[j];
            
            let mut p = (0, 0);
            if y11 >= min(y21, y22) && y11 <= max(y21, y22) && x21 >= min(x11, x12) && x21 <= max(x11, x12) {
                p = (x21, y11);
            }
            if y21 >= min(y11, y12) && y21 <= max(y11, y12) && x11 >= min(x21, x22) && x11 <= max(x21, x22) {
                p = (x11, y21);
            }
            
            if p == (0, 0) {
                continue;
            }
            points.push(p);
        }
    }
    
    return points;
}

fn calc_distance(wire: &Vec<(i32, i32)>, p: (i32, i32)) -> i32 {
    let mut d = 0;
    
    for i in 1..wire.len() {
        let (px, py) = p;
        let (x1, y1) = wire[i - 1];
        let (x2, y2) = wire[i];
        if (px == x1 && py >= min(y1, y2) && py <= max(y1, y2)) || (py == y1 && px >= min(x1, x2) && px <= max(x1, x2)) {
            d += (px - x1).abs() + (py - y1).abs();
            break;
        }
        d += (x2 - x1).abs() + (y2 - y1).abs();
    }
    
    return d;
}

fn main() {
    let mut reader = BufReader::new(File::open("input.txt").unwrap());
    
    let mut line1 = "".to_string();
    reader.read_line(&mut line1).expect("Failed to read input data");
    let wire1 = parse_wire_coords(line1);
        
    let mut line2 = "".to_string();
    reader.read_line(&mut line2).expect("Failed to read input data");
    let wire2 = parse_wire_coords(line2);
    
    let points = find_intersections(&wire1, &wire2);    
    let mut min_dist = std::i32::MAX;
    
    for p in &points {
        let d1 = calc_distance(&wire1, *p);
        let d2 = calc_distance(&wire2, *p);
        if d1 + d2 < min_dist {
            min_dist = d1 + d2;
        }
    }
    
    println!("{}", min_dist);
}