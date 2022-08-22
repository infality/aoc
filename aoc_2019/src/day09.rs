use std::collections::HashMap;

fn get(map: &HashMap<usize, isize>, i: usize) -> isize {
    *map.get(&i).unwrap_or(&0)
}

pub fn task1(input: &str) {
    let mut fields: HashMap<usize, isize> = HashMap::new();
    for (i, s) in input.split(',').enumerate() {
        fields.insert(i, s.parse().unwrap());
    }

    let mut counter = 0;
    let mut relative_base = 0isize;
    loop {
        let op_code = get(&fields, counter);
        if op_code == 99 {
            break;
        }

        let op1 = match digit_at_pos(op_code as usize, 2) {
            0 => get(&fields, get(&fields, counter + 1) as usize),
            1 => get(&fields, counter + 1),
            2 => get(
                &fields,
                (relative_base + get(&fields, counter + 1)) as usize,
            ),
            _ => panic!(),
        };

        let mut op2 = 0;
        if op_code % 100 != 3 && op_code % 100 != 4 && op_code % 100 != 9 {
            op2 = match digit_at_pos(op_code as usize, 3) {
                0 => get(&fields, get(&fields, counter + 2) as usize),
                1 => get(&fields, counter + 2),
                2 => get(
                    &fields,
                    (relative_base + get(&fields, counter + 2)) as usize,
                ),
                _ => panic!(),
            };
        }

        match op_code % 100 {
            1 => {
                let dest = get(&fields, counter + 3)
                    + if digit_at_pos(op_code as usize, 4) == 2 {
                        relative_base
                    } else {
                        0
                    };

                fields.insert(dest as usize, op1 + op2);
                counter += 4;
            }
            2 => {
                let dest = get(&fields, counter + 3)
                    + if digit_at_pos(op_code as usize, 4) == 2 {
                        relative_base
                    } else {
                        0
                    };

                fields.insert(dest as usize, op1 * op2);
                counter += 4;
            }
            3 => {
                println!("Input:");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                let dest = get(&fields, counter + 1)
                    + if digit_at_pos(op_code as usize, 2) == 2 {
                        relative_base
                    } else {
                        0
                    };
                fields.insert(dest as usize, input.trim().parse().unwrap());
                counter += 2;
            }
            4 => {
                println!("{}", op1);
                counter += 2;
            }
            5 => {
                if op1 != 0 {
                    counter = op2 as usize;
                } else {
                    counter += 3;
                }
            }
            6 => {
                if op1 == 0 {
                    counter = op2 as usize;
                } else {
                    counter += 3;
                }
            }
            7 => {
                let dest = get(&fields, counter + 3)
                    + if digit_at_pos(op_code as usize, 4) == 2 {
                        relative_base
                    } else {
                        0
                    };

                fields.insert(dest as usize, if op1 < op2 { 1 } else { 0 });
                counter += 4;
            }
            8 => {
                let dest = get(&fields, counter + 3)
                    + if digit_at_pos(op_code as usize, 4) == 2 {
                        relative_base
                    } else {
                        0
                    };

                fields.insert(dest as usize, if op1 == op2 { 1 } else { 0 });
                counter += 4;
            }
            9 => {
                relative_base += op1;
                counter += 2;
            }
            _ => panic!("{}", op_code),
        }
    }
}

pub fn task2(_input: &str) {}

fn digit_at_pos(num: usize, pos: usize) -> usize {
    (num / 10_usize.pow(pos as u32)) % 10
}
