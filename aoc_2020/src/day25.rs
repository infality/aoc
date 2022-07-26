pub fn task1(input: &str) {
    let mut result = 0usize;

    let card_pub = input.split_once('\n').unwrap().0.parse::<usize>().unwrap();
    let door_pub = input.split_once('\n').unwrap().1.parse::<usize>().unwrap();

    let mut card_loops = 0;
    let mut door_loops = 0;

    let mut value = 1;
    while value != card_pub {
        value = transform(value, 7);
        card_loops += 1;
    }

    let mut value = 1;
    while value != door_pub {
        value = transform(value, 7);
        door_loops += 1;
    }

    let mut encryption = 1;
    for _ in 0..door_loops {
        encryption = transform(encryption, card_pub);
    }

    result = encryption;

    println!("Result: {}", result);
}

fn transform(value: usize, subject_num: usize) -> usize {
    (value * subject_num) % 20201227
}

pub fn task2(input: &str) {
    let mut result = 0usize;
    println!("Result: {}", result);
}
