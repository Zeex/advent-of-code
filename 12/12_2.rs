use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_pos(s: &str) -> Vec<i32> {
    let coords_str = s.replace("<", "").replace(">", "").replace(" ", "");
    let mut p = vec![0, 0, 0];
    for c in coords_str.split(",") {
        let (name, value) = c.split_at(1);
        let value = value.replace("=", "").parse::<i32>().unwrap();
        match name {
            "x" => p[0] = value,
            "y" => p[1] = value,
            "z" => p[2] = value,
            _ => {}
        }
    }
    return p;
}

fn vec_eq(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    return v1.len() == v2.len()
        && v1.iter().zip(v2.iter()).filter(|&(x, y)| x == y).count() == v1.len();
}

fn main() {
    let mut pos: Vec<Vec<i32>> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| read_pos(&line.unwrap()))
        .collect();
    let initial_pos = pos.clone();
    let mut vel: Vec<Vec<i32>> = vec![vec![0, 0, 0]; pos.len()];
    let initial_vel = vel.clone();
    let mut periods: Vec<u64> = vec![0, 0, 0];
    let mut max_period = 0;
    
    for d in 0..3 {
        let mut n = 0;
        loop {
            for i in 0..pos.len() {
                for j in (i + 1)..pos.len() {
                    match pos[i][d] - pos[j][d] {
                        diff if diff < 0 => { vel[i][d] += 1; vel[j][d] -= 1; }
                        diff if diff > 0 => { vel[i][d] -= 1; vel[j][d] += 1; }
                        _ => {}
                    }
                }
            }
            for i in 0..pos.len() {
                pos[i][d] += vel[i][d];
            }
            n += 1;
            if vec_eq(&initial_pos.iter().map(|p| p[d]).collect(), &pos.iter().map(|p| p[d]).collect())
                    && vec_eq(&initial_vel.iter().map(|p| p[d]).collect(), &vel.iter().map(|p| p[d]).collect()) {
                periods[d] = n as u64;
                max_period = max(n, max_period);
                println!("Dimension {} period: {}", d, n);
                break;
            }
        }   
    }
    
    let mut n: u64 = 0;
    loop {
        n += max_period;
        if n % periods[0] == 0 && n % periods[1] == 0 && n % periods[2] == 0 {
            println!("{}", n);
            break;
        }
    }
}