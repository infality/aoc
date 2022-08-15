pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut fields: Vec<usize> = Vec::new();
    for s in input.split(',') {
        fields.push(s.parse().unwrap());
    }

    fields[1] = 12;
    fields[2] = 2;

    let mut counter = 0;
    loop {
        let op_code = fields[counter];
        let op1 = fields[counter + 1];
        let op2 = fields[counter + 2];
        let dest = fields[counter + 3];
        match op_code {
            1 => {
                fields[dest] = fields[op1] + fields[op2];
            }
            2 => {
                fields[dest] = fields[op1] * fields[op2];
            }
            99 => {
                break;
            }
            _ => panic!(),
        }
        counter += 4;
    }

    result = fields[0];

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut orig_fields: Vec<usize> = Vec::new();
    for s in input.split(',') {
        orig_fields.push(s.parse().unwrap());
    }

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut fields = orig_fields.clone();
            fields[1] = noun;
            fields[2] = verb;

            let mut counter = 0;
            loop {
                let op_code = fields[counter];
                let op1 = fields[counter + 1];
                let op2 = fields[counter + 2];
                let dest = fields[counter + 3];
                match op_code {
                    1 => {
                        fields[dest] = fields[op1] + fields[op2];
                    }
                    2 => {
                        fields[dest] = fields[op1] * fields[op2];
                    }
                    99 => {
                        break;
                    }
                    _ => panic!(),
                }
                counter += 4;
            }

            if fields[0] == 19690720 {
                result = 100 * noun + verb;
                println!("Result: {}", result);
                return;
            }
        }
    }
}
