pub fn task1(input: &str) {
    let mut positions = Vec::<usize>::new();

    for num in input.split(",") {
        positions.push(num.parse().unwrap());
    }
    positions.sort();

    let end_pos = positions[positions.len() / 2];

    let mut result = 0usize;
    for pos in positions {
        if end_pos > pos {
            result += end_pos - pos;
        } else {
            result += pos - end_pos;
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut positions = Vec::<usize>::new();

    for num in input.split(",") {
        positions.push(num.parse().unwrap());
    }
    positions.sort();

    let end_pos = positions.iter().sum::<usize>() / positions.len();

    let mut result = 0usize;
    for pos in positions {
        let n;
        if end_pos > pos {
            n = end_pos - pos;
        } else {
            n = pos - end_pos;
        }
        result += (n.pow(2) + n) / 2
    }

    println!("Result: {}", result);
}
