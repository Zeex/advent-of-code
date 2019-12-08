use std::fs;

fn main() {
    let width = 25;
    let height = 6;
    let input = fs::read_to_string("input.txt").unwrap();
    let image: Vec<_> = input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();
    let layers: Vec<_> = image.chunks(width * height).map(|layer| layer.to_vec()).collect();
    
    let mut min_layer_index = 0;
    let mut min_count = std::u32::MAX;
    for (i, layer) in layers.iter().enumerate() {
        let count = layer.to_vec().iter().fold(0, |count, &x| count + (x == 0) as u32);
        if count < min_count {
            min_count = count;
            min_layer_index = i;
        }
    }
        
    let min_layer = &layers[min_layer_index];
    let one_count = min_layer.iter().filter(|x| **x == 1).count();
    let two_count = min_layer.iter().filter(|x| **x == 2).count();
    println!("{}", one_count * two_count);
}