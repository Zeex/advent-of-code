use std::cmp::{min, max};
use std::fs;
use std::f32;

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

fn find_asteroids(map: &Vec<Vec<char>>) -> Vec<(i32, i32)>{
    let mut asteroids = vec!();
    for i in 0..map[0].len() {
        for j in 0..map.len() {
            if map[j][i] == '#' {
                asteroids.push((i as i32, j as i32));
            }
        }
    }
    return asteroids;
}

fn find_detectable_asteroids(map: &Vec<Vec<char>>, station: (i32, i32)) -> Vec<(i32, i32)> {
    let mut asteroids = vec!();
    let (station_x, station_y) = station;
    for i in 0..map[0].len() {
        for j in 0..map.len() {
            let asteroid = (i as i32, j as i32);
            let (x, y) = asteroid;
            if map[j][i] == '#' && (x != station_x || y != station_y) {
                if find_crossing_points(map, station, asteroid).len() == 0 {
                    asteroids.push(asteroid);
                }
            }
        }
    }
    return asteroids;
}

fn main() {
    let mut map = fs::read_to_string("input.txt")
        .unwrap()
        .split("\r\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut asteroids = find_asteroids(&map);
    let mut max_asteroids = 0;
    let mut station = (0, 0);
    for p in &asteroids {
        let score = find_detectable_asteroids(&map, *p).len();
        if score > max_asteroids {
            max_asteroids = score;
            station = *p;
        }
    }
    
    println!("Station: {:?}", station);
    let (station_x, station_y) = station;
    map[station_y as usize][station_x as usize] = '.';
    
    let mut asteroid_count = 0;
    'outer: while asteroids.len() > 0 {
        let mut asteroids_to_vaporize = find_detectable_asteroids(&map, station);
        asteroids_to_vaporize.sort_by(|&(x1, y1), &(x2, y2)| {
            let (sx1, sy1) = ((x1 - station_x) as f32, (station_y - y1) as f32);
            let (sx2, sy2) = ((x2 - station_x) as f32, (station_y - y2) as f32);
            let angle1 = -(sy1 / sx1).atan() + (if sx1 < 0.0 {f32::consts::PI} else {0.0});
            let angle2 = -(sy2 / sx2).atan() + (if sx2 < 0.0 {f32::consts::PI} else {0.0});
            return angle1.partial_cmp(&angle2).unwrap();
        });
        println!("Vaporize: {:?} {}", asteroids_to_vaporize, asteroids_to_vaporize.len());
        for (x, y) in asteroids_to_vaporize {
            map[y as usize][x as usize] = '.';
            asteroid_count += 1;
            match asteroid_count {
                1 | 2 | 3 | 10 | 20 | 50 | 100 | 199 | 200 | 201 | 299 => println!("{}: {:?}", asteroid_count, (x, y)),
                _ => {}
            }
            if asteroid_count == 200 {
                println!("{}", x * 100 + y);
                break 'outer;
            }
        }
        asteroids = find_asteroids(&map);
    }
}