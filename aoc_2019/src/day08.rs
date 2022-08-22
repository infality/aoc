pub fn task1(input: &str) {
    let mut result = 0usize;

    let width = 25;
    let height = 6;
    let mut layers = Vec::new();
    let mut current_layer = [[0; 25]; 6];
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        for y in 0..height {
            for x in 0..width {
                current_layer[y][x] = chars[i].to_digit(10).unwrap();
                i += 1;
            }
        }
        layers.push(current_layer);
        current_layer = [[0; 25]; 6];
    }

    let mut min_zeros = usize::MAX;
    for layer in layers.iter() {
        let mut zeros = 0;
        let mut ones = 0;
        let mut twos = 0;
        for y in 0..height {
            for x in 0..width {
                if layer[y][x] == 0 {
                    zeros += 1;
                } else if layer[y][x] == 1 {
                    ones += 1;
                } else if layer[y][x] == 2 {
                    twos += 1;
                }
            }
        }
        if zeros < min_zeros {
            min_zeros = zeros;
            result = ones * twos;
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let width = 25;
    let height = 6;
    let mut layers = Vec::new();
    let mut current_layer = [[0; 25]; 6];
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        for y in 0..height {
            for x in 0..width {
                current_layer[y][x] = chars[i].to_digit(10).unwrap();
                i += 1;
            }
        }
        layers.push(current_layer);
        current_layer = [[0; 25]; 6];
    }

    for y in 0..height {
        for x in 0..width {
            for layer in layers.iter() {
                if layer[y][x] != 2 {
                    print!("{}", if layer[y][x] == 1 { 'X' } else { ' ' });
                    break;
                }
            }
        }
        println!();
    }
}
