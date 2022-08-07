pub fn task1(input: &str) {
    let mut result = 0usize;

    for line in input.lines() {
        let value: usize = line.parse().unwrap();
        result += value / 3 - 2;
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    for line in input.lines() {
        let mut value: usize = line.parse().unwrap();
        while value > 8 {
            value = value / 3 - 2;
            result += value;
        }
    }

    println!("Result: {}", result);
}
