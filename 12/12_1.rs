use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

fn read_pos(s: &str) -> Point {
    let coords_str = s.replace("<", "").replace(">", "").replace(" ", "");
    let mut p = Point {x: 0, y: 0, z: 0};
    for c in coords_str.split(",") {
        let (name, value) = c.split_at(1);
        let value = value.replace("=", "").parse::<i32>().unwrap();
        match name {
            "x" => p.x = value,
            "y" => p.y = value,
            "z" => p.z = value,
            _ => {}
        }
    }
    return p;
}

fn print_pos(p: &Point) {
   print!("<x={: >4}, y={: >4}, z={: >4}>", p.x, p.y, p.z); 
}

fn main() {
    const MAX_STEPS: usize = 1000;
    
    let mut pos: Vec<Point> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| read_pos(&line.unwrap()))
        .collect();
    let mut vel: Vec<Point> = vec![Point {x: 0, y: 0, z: 0}; pos.len()];
    
    for step in 0..(MAX_STEPS + 1) {
        println!("After step {}:", step);
        for i in 0..pos.len() {
            print!("pos=");
            print_pos(&pos[i]);
            print!(", vel=");
            print_pos(&vel[i]);
            print!("\n");
        }
        print!("\n");
        
        if step == MAX_STEPS {
            break;
        }
        
        for i in 0..pos.len() {
            for j in (i + 1)..pos.len() {
                match pos[i].x - pos[j].x {
                    d if d < 0 => { vel[i].x += 1; vel[j].x -= 1; }
                    d if d > 0 => { vel[i].x -= 1; vel[j].x += 1; }
                    _ => {}
                }
                match pos[i].y - pos[j].y {
                    d if d < 0 => { vel[i].y += 1; vel[j].y -= 1; }
                    d if d > 0 => { vel[i].y -= 1; vel[j].y += 1; }
                    _ => {}
                }
                match pos[i].z - pos[j].z {
                    d if d < 0 => { vel[i].z += 1; vel[j].z -= 1; }
                    d if d > 0 => { vel[i].z -= 1; vel[j].z += 1; }
                    _ => {}
                }
            }
        }
        for i in 0..pos.len() {
            pos[i].x += vel[i].x;
            pos[i].y += vel[i].y;
            pos[i].z += vel[i].z;
        }
    }
    
    println!("Energy after {} steps:", MAX_STEPS);
    let mut total_energy = 0;
    
    for i in 0..pos.len() {
        let pot = pos[i].x.abs() + pos[i].y.abs() + pos[i].z.abs();
        let kin = vel[i].x.abs() + vel[i].y.abs() + vel[i].z.abs();
        println!("pot = {}, kin = {}", pot, kin);
        total_energy += pot * kin;
    }
    
    println!("Sum of total energy: {}", total_energy);
    
    // println!("{:?}", vel);
}