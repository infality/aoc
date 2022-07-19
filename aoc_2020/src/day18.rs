pub fn task1(input: &str) {
    let mut result = 0usize;
    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    for line in input.lines() {
        let mut eq = line.chars().filter(|x| *x != ' ').collect::<Vec<char>>();

        let mut i = 0;
        while i < eq.len() {
            if eq[i] == '+' {
                if eq[i - 1].is_ascii_digit() {
                    eq.insert(i - 1, '(');
                    i += 1;
                } else if eq[i - 1] == ')' {
                    let mut parentheses = 1;
                    for j in (0..i - 1).rev() {
                        if eq[j] == ')' {
                            parentheses += 1;
                        } else if eq[j] == '(' {
                            parentheses -= 1;
                        }
                        if parentheses == 0 {
                            eq.insert(j, '(');
                            i += 1;
                            break;
                        }
                        if j == 0 {
                            panic!();
                        }
                    }
                } else {
                    panic!();
                }

                if eq[i + 1].is_ascii_digit() {
                    eq.insert(i + 2, ')');
                } else if eq[i + 1] == '(' {
                    let mut parentheses = 1;
                    for j in i + 2..eq.len() {
                        if eq[j] == '(' {
                            parentheses += 1;
                        } else if eq[j] == ')' {
                            parentheses -= 1;
                        }
                        if parentheses == 0 {
                            eq.insert(j, ')');
                            break;
                        }
                        if j == eq.len() - 1 {
                            panic!();
                        }
                    }
                } else {
                    panic!();
                }
            }
            i += 1;
        }
        print!("{}+", eq.iter().collect::<String>());
    }
}
