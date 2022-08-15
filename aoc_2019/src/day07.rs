use itertools::Itertools;

pub fn task1(input: &str) {
    let mut result = isize::MIN;

    let mut fields: Vec<isize> = Vec::new();
    for s in input.split(',') {
        fields.push(s.parse().unwrap());
    }

    for combination in (0..5).permutations(5) {
        let mut inputs = [0; 6];
        for i in 0..5 {
            inputs[i + 1] =
                run(&mut (fields.clone(), 0), &vec![combination[i], inputs[i]]).unwrap();
        }
        result = result.max(inputs[5]);
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0isize;

    let mut fields: Vec<isize> = Vec::new();
    for s in input.split(',') {
        fields.push(s.parse().unwrap());
    }

    for combination in (5..10).permutations(5) {
        let mut states = vec![(fields.clone(), 0); 5];
        let mut inputs = [0; 5];
        let mut first_iter = true;
        loop {
            dbg!(states[0].1);
            let mut finished = false;
            for i in 0..5 {
                let output = run(
                    &mut states[i],
                    &if first_iter {
                        vec![combination[i], inputs[i]]
                    } else {
                        vec![inputs[i]]
                    },
                );
                if output.is_some() {
                    inputs[(i + 1) % 5] = output.unwrap();
                } else {
                    result = result.max(inputs[0]);
                    finished = true;
                    break;
                }
            }
            first_iter = false;
            if finished {
                break;
            }
        }
    }

    println!("Result: {}", result);
}

fn run(f: &mut (Vec<isize>, usize), input: &[isize]) -> Option<isize> {
    let mut input_counter = 0;
    let fields = &mut f.0;
    let counter = &mut f.1;
    loop {
        let op_code = fields[*counter];
        if op_code == 99 {
            return None;
        }

        let op1_lit = digit_at_pos(op_code as usize, 2) == 1;
        let op2_lit = digit_at_pos(op_code as usize, 3) == 1;

        let op1 = if op1_lit {
            fields[*counter + 1]
        } else {
            fields[fields[*counter + 1] as usize]
        };
        let op2 = if op_code % 100 == 3 || op_code % 100 == 4 {
            0
        } else if op2_lit {
            fields[*counter + 2]
        } else {
            fields[fields[*counter + 2] as usize]
        };

        match op_code % 100 {
            1 => {
                let dest = fields[*counter + 3] as usize;

                fields[dest] = op1 + op2;
                *counter += 4;
            }
            2 => {
                let dest = fields[*counter + 3] as usize;

                fields[dest] = op1 * op2;
                *counter += 4;
            }
            3 => {
                let pos = fields[*counter + 1] as usize;
                fields[pos] = input[input_counter];
                input_counter += 1;
                *counter += 2;
            }
            4 => {
                *counter += 2;
                return Some(op1);
            }
            5 => {
                if op1 != 0 {
                    *counter = op2 as usize;
                } else {
                    *counter += 3;
                }
            }
            6 => {
                if op1 == 0 {
                    *counter = op2 as usize;
                } else {
                    *counter += 3;
                }
            }
            7 => {
                let dest = fields[*counter + 3] as usize;

                fields[dest] = if op1 < op2 { 1 } else { 0 };
                *counter += 4;
            }
            8 => {
                let dest = fields[*counter + 3] as usize;

                fields[dest] = if op1 == op2 { 1 } else { 0 };
                *counter += 4;
            }
            _ => panic!("{}", op_code),
        }
    }
}

fn digit_at_pos(num: usize, pos: usize) -> usize {
    (num / 10_usize.pow(pos as u32)) % 10
}
