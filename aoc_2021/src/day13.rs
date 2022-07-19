pub fn task1(input: &str) {
    let mut result = 0usize;
    let file_parts = input.split_once("\n\n").unwrap();

    let mut data = Vec::new();
    for line in file_parts.0.split('\n') {
        let entry = line.split_once(',').unwrap();
        data.push((
            entry.0.to_string().parse::<usize>().unwrap(),
            entry.1.to_string().parse::<usize>().unwrap(),
        ));
    }

    for instruction in file_parts.1.split('\n') {
        let direction = instruction.chars().nth(11).unwrap();
        let pos = instruction
            .split_once('=')
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();

        if direction == 'x' {
            for i in (0..data.len()).rev() {
                if data[i].0 < pos {
                    continue;
                }
                let new_x = 2 * pos - data[i].0;
                if data.iter().any(|x| x.0 == new_x && x.1 == data[i].1) {
                    data.remove(i);
                } else {
                    data[i].0 = new_x;
                }
            }
        } else {
            for i in (0..data.len()).rev() {
                if data[i].1 < pos {
                    continue;
                }
                let new_y = 2 * pos - data[i].1;
                if data.iter().any(|x| x.1 == new_y && x.0 == data[i].0) {
                    data.remove(i);
                } else {
                    data[i].1 = new_y;
                }
            }
        }
    }
    result = data.len();

    for y in 0..=data.iter().max_by_key(|x| x.1).unwrap().1 {
        for x in 0..=data.iter().max_by_key(|x| x.0).unwrap().0 {
            if data.iter().any(|i| i.0 == x && i.1 == y) {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;
    println!("Result: {}", result);
}
