pub fn task1(input: &str) {
    let mut result = 0usize;

    for line in input.lines() {
        let first = line.chars().find(|x| x.is_digit(10)).unwrap();
        let last = line.chars().rev().find(|x| x.is_digit(10)).unwrap();
        result += format!("{first}{last}").parse::<usize>().unwrap();
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in input.lines() {
        let mut first = String::new();
        let mut last = String::new();

        let chars = line.chars().collect::<Vec<char>>();
        for i in 0..chars.len() {
            if chars[i].is_digit(10) {
                first = chars[i].to_string();
                break;
            }

            let rest = chars.len() - i;
            for (num, number) in numbers.iter().enumerate() {
                if number.len() > rest {
                    continue;
                }
                if line[i..].starts_with(number) {
                    first = (num + 1).to_string();
                    break;
                }
            }
            if !first.is_empty() {
                break;
            }
        }

        for i in (0..chars.len()).rev() {
            if chars[i].is_digit(10) {
                last = chars[i].to_string();
                break;
            }

            let rest = chars.len() - i;
            for (num, number) in numbers.iter().enumerate() {
                if number.len() > rest {
                    continue;
                }
                if line[i..].starts_with(number) {
                    last = (num + 1).to_string();
                    break;
                }
            }
            if !last.is_empty() {
                break;
            }
        }

        result += format!("{first}{last}").parse::<usize>().unwrap();
    }

    println!("Result: {}", result);
}
