pub fn task1(input: &str) {
    let mut result = 0usize;
    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut data = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for entry in line.chars() {
            row.push(entry.to_string().parse::<usize>().unwrap());
        }
        data.push(row);
    }
    let mut data2 = data.clone();

    for bit in 0..data[0].len() {
        let rows = data.len();
        let mut ones = 0;
        for row in 0..rows {
            if data[row][bit] == 1 {
                ones += 1;
            }
        }
        for row in (0..rows).rev() {
            if ones >= (rows + 1) / 2 {
                if data[row][bit] == 0 {
                    data.remove(row);
                }
            } else {
                if data[row][bit] == 1 {
                    data.remove(row);
                }
            }
        }
        if data.len() == 1 {
            for (i, val) in data[0].iter().rev().enumerate() {
                if *val == 1 {
                    result += 2usize.pow(i as u32);
                }
            }
            break;
        }
    }

    for bit in 0..data2[0].len() {
        let rows = data2.len();
        let mut ones = 0;
        for row in 0..rows {
            if data2[row][bit] == 1 {
                ones += 1;
            }
        }
        for row in (0..rows).rev() {
            if ones >= (rows + 1) / 2 {
                if data2[row][bit] == 1 {
                    data2.remove(row);
                }
            } else {
                if data2[row][bit] == 0 {
                    data2.remove(row);
                }
            }
        }
        if data2.len() == 1 {
            let mut value = 0;
            for (i, val) in data2[0].iter().rev().enumerate() {
                if *val == 1 {
                    value += 2usize.pow(i as u32);
                }
            }
            result *= value;
            break;
        }
    }

    println!("Result: {}", result);
}
