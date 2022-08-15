pub fn task1(input: &str) {
    let mut fields: Vec<isize> = Vec::new();
    for s in input.split(',') {
        fields.push(s.parse().unwrap());
    }

    let mut counter = 0;
    loop {
        let op_code = fields[counter];
        if op_code == 99 {
            break;
        }

        let op1_lit = digit_at_pos(op_code as usize, 2) == 1;
        let op2_lit = digit_at_pos(op_code as usize, 3) == 1;

        let op1 = if op1_lit {
            fields[counter + 1]
        } else {
            fields[fields[counter + 1] as usize]
        };

        match op_code % 100 {
            1 => {
                let op2 = if op2_lit {
                    fields[counter + 2]
                } else {
                    fields[fields[counter + 2] as usize]
                };
                let dest = fields[counter + 3] as usize;

                fields[dest] = op1 + op2;
                counter += 4;
            }
            2 => {
                let op2 = if op2_lit {
                    fields[counter + 2]
                } else {
                    fields[fields[counter + 2] as usize]
                };
                let dest = fields[counter + 3] as usize;

                fields[dest] = op1 * op2;
                counter += 4;
            }
            3 => {
                println!("Input:");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                let pos = fields[counter + 1] as usize;
                fields[pos] = input.trim().parse().unwrap();
                counter += 2;
            }
            4 => {
                println!("{}", op1);
                counter += 2;
            }
            _ => panic!("{}", op_code),
        }
    }
}

pub fn task2(input: &str) {
    let mut fields: Vec<isize> = Vec::new();
    for s in input.split(',') {
        fields.push(s.parse().unwrap());
    }

    let mut counter = 0;
    loop {
        let op_code = fields[counter];
        if op_code == 99 {
            break;
        }

        let op1_lit = digit_at_pos(op_code as usize, 2) == 1;
        let op2_lit = digit_at_pos(op_code as usize, 3) == 1;

        let op1 = if op1_lit {
            fields[counter + 1]
        } else {
            fields[fields[counter + 1] as usize]
        };
        let op2 = if op_code % 100 == 3 || op_code % 100 == 4 {
            0
        } else if op2_lit {
            fields[counter + 2]
        } else {
            fields[fields[counter + 2] as usize]
        };

        match op_code % 100 {
            1 => {
                let dest = fields[counter + 3] as usize;

                fields[dest] = op1 + op2;
                counter += 4;
            }
            2 => {
                let dest = fields[counter + 3] as usize;

                fields[dest] = op1 * op2;
                counter += 4;
            }
            3 => {
                println!("Input:");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                let pos = fields[counter + 1] as usize;
                fields[pos] = input.trim().parse().unwrap();
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
                let dest = fields[counter + 3] as usize;

                fields[dest] = if op1 < op2 { 1 } else { 0 };
                counter += 4;
            }
            8 => {
                let dest = fields[counter + 3] as usize;

                fields[dest] = if op1 == op2 { 1 } else { 0 };
                counter += 4;
            }
            _ => panic!("{}", op_code),
        }
    }
}

fn digit_at_pos(num: usize, pos: usize) -> usize {
    (num / 10_usize.pow(pos as u32)) % 10
}
