use std::cmp::{min, max};
use std::fs;

fn find_crossing_points(map: &Vec<Vec<char>>, a: (i32, i32), b: (i32, i32)) -> Vec<(i32, i32)> {
    let mut points = vec!();
    let ((x1, y1), (x2, y2)) = (a, b);
    let ab_dist_sq = ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)) as f32;
    for x in min(x1, x2)..=max(x1, x2) {
        for y in min(y1, y2)..=max(y1, y2) {
            let c = (x, y);
            if map[y as usize][x as usize] == '#' && c != a && c != b {
                let a_dist_sq = ((x1 - x) * (x1 - x) + (y1 - y) * (y1 - y)) as f32;
                let b_dist_sq = ((x2 - x) * (x2 - x) + (y2 - y) * (y2 - y)) as f32;
                if (a_dist_sq + b_dist_sq + 2_f32 * (a_dist_sq * b_dist_sq).sqrt() - ab_dist_sq).abs() <= 1E-6 {
                    points.push(c);
                }
            }
        }
    }
    return points;
}

fn calc_station_score(map: &Vec<Vec<char>>, station: (i32, i32)) -> u32 {
    let mut count = 0;
    let (station_x, station_y) = station;
    for i in 0..map[0].len() {
        for j in 0..map.len() {
            let asteroid = (i as i32, j as i32);
            let (x, y) = asteroid;
            if map[j][i] == '#' && (x != station_x || y != station_y) {
                if find_crossing_points(map, station, asteroid).len() == 0 {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn main() {
    let map = fs::read_to_string("input.txt")
        .unwrap()
        .split("\r\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut max_asteroids = 0;
    let mut max_asteroids_station = (0, 0);
    for i in 0..map[0].len() {
        for j in 0..map.len() {
            if map[j][i] == '#' {
                let station_coord = (i as i32, j as i32);
                println!("*** Possible station: {:?} ***", station_coord);
                let score = calc_station_score(&map, station_coord);
                println!("Can detect {} asteroids", score);
                if score > max_asteroids {
                    max_asteroids = score;
                    max_asteroids_station = station_coord;
                }
            }
        }
    }

    println!("{:?} can detect {} asteroids", max_asteroids_station, max_asteroids);
}