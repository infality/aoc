pub fn task1(input: &str) {
    let mut result = 0usize;

    let bits = input.chars().map(to_binary).collect::<String>();
    result = parse_packet(&bits).1;

    println!("Result: {}", result);
}

fn parse_packet(data: &str) -> (usize, usize) {
    let mut result = 0;
    let version = usize::from_str_radix(&data[0..3], 2).unwrap();
    println!("{}", version);
    let type_id = usize::from_str_radix(&data[3..6], 2).unwrap();
    result += version;

    if type_id == 4 {
        let mut literal_binary = String::new();
        let mut pos = 6;
        loop {
            literal_binary.extend(data.chars().skip(pos + 1).take(4));
            if get_char(data, pos) == '0' {
                return (pos + 5, result);
            }
            pos += 5;
        }
    }

    let length_id = get_char(data, 6);
    if length_id == '0' {
        let packets_length = usize::from_str_radix(&data[7..22], 2).unwrap();
        let mut pos = 22;
        while pos < 22 + packets_length {
            let res = parse_packet(&data[pos..]);
            pos += res.0;
            result += res.1;
        }
        return (pos, result);
    } else {
        let packets_count = usize::from_str_radix(&data[7..18], 2).unwrap();
        let mut pos = 18;
        for _ in 0..packets_count {
            let res = parse_packet(&data[pos..]);
            pos += res.0;
            result += res.1;
        }
        return (pos, result);
    }
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let bits = input.chars().map(to_binary).collect::<String>();
    result = parse_packet2(&bits).1;

    println!("Result: {}", result);
}

fn parse_packet2(data: &str) -> (usize, usize) {
    println!("{}", data);
    let _version = usize::from_str_radix(&data[0..3], 2).unwrap();
    let type_id = usize::from_str_radix(&data[3..6], 2).unwrap();

    if type_id == 4 {
        let mut literal_binary = String::new();
        let mut pos = 6;
        loop {
            literal_binary.extend(data.chars().skip(pos + 1).take(4));
            if get_char(data, pos) == '0' {
                return (pos + 5, usize::from_str_radix(&literal_binary, 2).unwrap());
            }
            pos += 5;
        }
    }

    let mut values = Vec::new();
    let length_id = get_char(data, 6);
    let mut pos;
    if length_id == '0' {
        let packets_length = usize::from_str_radix(&data[7..22], 2).unwrap();
        pos = 22;
        while pos < 22 + packets_length {
            let res = parse_packet2(&data[pos..]);
            pos += res.0;
            values.push(res.1);
        }
    } else {
        let packets_count = usize::from_str_radix(&data[7..18], 2).unwrap();
        pos = 18;
        for _ in 0..packets_count {
            let res = parse_packet2(&data[pos..]);
            pos += res.0;
            values.push(res.1);
        }
    }

    let mut result = *values.first().unwrap();
    for value in values.iter().skip(1) {
        match type_id {
            0 => result += value,
            1 => result *= value,
            2 => result = result.min(*value),
            3 => result = result.max(*value),
            5 => result = if result > *value { 1 } else { 0 },
            6 => result = if result < *value { 1 } else { 0 },
            7 => result = if result == *value { 1 } else { 0 },
            _ => panic!(),
        }
    }

    return (pos, result);
}

fn get_char(data: &str, pos: usize) -> char {
    data.chars().nth(pos).unwrap()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}
