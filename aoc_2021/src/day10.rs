pub fn task1(input: &str) {
    let mut result = 0usize;

    for line in input.lines() {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        result += 3;
                        break;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        result += 57;
                        break;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        result += 1197;
                        break;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        result += 25137;
                        break;
                    }
                }
                _ => panic!(),
            }
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut results = Vec::new();
    for line in input.lines() {
        let mut result = 0usize;
        let mut stack = Vec::new();
        let mut corrupt = false;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.pop().unwrap() != '(' {
                        corrupt = true;
                        break;
                    }
                }
                ']' => {
                    if stack.pop().unwrap() != '[' {
                        corrupt = true;
                        break;
                    }
                }
                '}' => {
                    if stack.pop().unwrap() != '{' {
                        corrupt = true;
                        break;
                    }
                }
                '>' => {
                    if stack.pop().unwrap() != '<' {
                        corrupt = true;
                        break;
                    }
                }
                _ => panic!(),
            }
        }

        if corrupt {
            continue;
        }

        for c in stack.iter().rev() {
            result *= 5;
            result += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!(),
            }
        }
        results.push(result);
    }

    results.sort();
    println!("Result: {}", results[results.len() / 2]);
}
