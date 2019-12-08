use std::fs;

fn main() {
    let width = 25;
    let height = 6;
    let input = fs::read_to_string("input.txt").unwrap();
    let image: Vec<_> = input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();
    let layers: Vec<_> = image.chunks(width * height).map(|layer| layer.to_vec()).collect();
    
    let mut final_layer = vec![0; width * height];
    for i in 0..(width * height) {
        for layer in layers.iter().rev() {
            if layer[i] != 2 {
                final_layer[i] = layer[i];
            }
        }
    }

    for i in 0..height {
        for j in 0..width {
            match final_layer[i * width + j] {
                1 => print!("O "),
                _ => print!("  ")
            }
        }
        println!("");
    }
}