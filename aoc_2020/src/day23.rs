pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut cups = [0; 9];
    for (i, c) in input.chars().enumerate() {
        cups[i] = c.to_string().parse().unwrap();
    }

    let mut current = 8;
    for _ in 0..100 {
        current = (current + 1) % 9;
        let picked = [
            cups[(current + 1) % 9],
            cups[(current + 2) % 9],
            cups[(current + 3) % 9],
        ];

        let mut small_max = None;
        let mut big_max = None;
        for i in 4..9 {
            let value = cups[(current + i) % 9];
            if value < cups[current] {
                small_max = Some(small_max.unwrap_or(0).max(value))
            } else {
                big_max = Some(big_max.unwrap_or(0).max(value))
            }
        }

        let destination = cups
            .iter()
            .position(|x| *x == small_max.unwrap_or_else(|| big_max.unwrap()))
            .unwrap();
        /* println!(
            "[{}]={} [{}]={} {:?}",
            current, cups[current], destination, cups[destination], cups
        ); */

        for i in 4..=((9 + destination - current) % 9) {
            cups[(current + i - 3) % 9] = cups[(current + i) % 9];
        }
        cups[(9 + destination - 2) % 9] = picked[0];
        cups[(9 + destination - 1) % 9] = picked[1];
        cups[(9 + destination) % 9] = picked[2];
    }

    let one = cups.iter().position(|x| *x == 1).unwrap();
    for i in 0..8 {
        result += cups[(one + 1 + i) % 9] * 10usize.pow(7 - i as u32);
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    // Array entry holds index of next cup
    let mut cups = [0; 1000001];
    let mut last_cup = 1000000;
    for c in input.chars() {
        let value = c.to_string().parse().unwrap();
        cups[last_cup] = value;
        last_cup = value;
    }
    cups[last_cup] = 10;
    for i in 10..1000000 {
        cups[i] = i + 1;
    }

    let mut current = 1000000;
    for _ in 0..10000000 {
        current = cups[current];
        let mut picked = [0; 3];
        picked[0] = cups[current];
        picked[1] = cups[picked[0]];
        picked[2] = cups[picked[1]];

        let mut destination = 0;
        for i in 1..5 {
            let value = if i < current {
                current - i
            } else {
                1000000 - i + current
            };
            if !picked.contains(&value) {
                destination = value;
                break;
            }
            if i == 4 {
                panic!();
            }
        }

        /* println!(
            "[{}]={} [{}]={} {:?}",
            current, cups[current], destination, cups[destination], cups
        ); */

        cups[current] = cups[picked[2]];
        cups[picked[2]] = cups[destination];
        cups[destination] = picked[0];
    }

    println!("{} {}", cups[1], cups[cups[1]]);
    result = cups[1] * cups[cups[1]];

    println!("Result: {}", result);
}
