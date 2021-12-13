use std::fs;

fn day_0101() {
    let file = fs::read_to_string("input/01.txt").unwrap();
    let mut previous = i32::MAX;
    let mut result = 0;
    for line in file.lines() {
        let current = line.parse::<i32>().unwrap();
        if current > previous {
            result += 1;
        }
        previous = current;
    }
    println!("0101: {}", result);
}

fn day_0102() {
    let file = fs::read_to_string("input/01.txt").unwrap();
    let mut result = 0;
    let mut lines = file.lines();
    let mut abc = [0; 3];
    for item in &mut abc {
        *item = lines.next().unwrap().parse::<i32>().unwrap();
    }
    let mut i = 0;
    for line in lines {
        let current = line.parse::<i32>().unwrap();
        if current > abc[i] {
            result += 1;
        }
        abc[i] = current;
        i = (i + 1) % 3;
    }
    println!("0102: {}", result);
}

fn day_0201() {
    let file = fs::read_to_string("input/02.txt").unwrap();
    let mut horizontal = 0;
    let mut vertical = 0;
    for line in file.lines() {
        let (command, position_str) = line.split_once(' ').unwrap();
        let position = position_str.parse::<i32>().unwrap();
        if command == "forward" {
            horizontal += position;
        } else if command == "up" {
            vertical -= position;
        } else if command == "down" {
            vertical += position;
        }
    }
    println!("0201: {}", horizontal * vertical);
}

fn day_0202() {
    let file = fs::read_to_string("input/02.txt").unwrap();
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    for line in file.lines() {
        let (command, position_str) = line.split_once(' ').unwrap();
        let position = position_str.parse::<i32>().unwrap();
        if command == "forward" {
            horizontal += position;
            vertical += aim * position;
        } else if command == "up" {
            aim -= position;
        } else if command == "down" {
            aim += position;
        }
    }
    println!("0202: {}", horizontal * vertical);
}

fn day_0301() {
    let file = fs::read_to_string("input/03.txt").unwrap();
    let mut bits = [0; 12];
    for line in file.lines() {
        for (i, bit) in bits.iter_mut().enumerate() {
            if line.chars().nth(i).unwrap() == '1' {
                *bit += 1;
            } else {
                *bit -= 1;
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut add = 1;
    for i in (0..12).rev() {
        if bits[i] > 0 {
            gamma += add;
        } else {
            epsilon += add;
        }
        add *= 2;
    }

    println!("0301: {}", gamma * epsilon);
}

fn filter(numbers: &[&str], index: usize, mc: bool) -> i32 {
    if numbers.len() == 1 {
        let mut result = 0;
        let mut add = 1;
        for i in (0..12).rev() {
            if numbers[0].chars().nth(i).unwrap() == '1' {
                result += add;
            }
            add *= 2;
        }
        return result;
    }

    let mut bit = 0;
    let mut numbers_one = Vec::new();
    let mut numbers_zero = Vec::new();
    for number in numbers {
        if number.chars().nth(index).unwrap() == '1' {
            numbers_one.push(*number);
            bit += 1;
        } else {
            numbers_zero.push(*number);
            bit -= 1;
        }
    }

    if (mc && bit >= 0) || (!mc && bit < 0) {
        filter(&numbers_one, index + 1, mc)
    } else {
        filter(&numbers_zero, index + 1, mc)
    }
}

fn day_0302() {
    let file = fs::read_to_string("input/03.txt").unwrap();
    let mut numbers = Vec::new();
    for line in file.lines() {
        numbers.push(line);
    }

    let oxygen = filter(&numbers, 0, true);
    let co2 = filter(&numbers, 0, false);

    println!("0302: {}", oxygen * co2);
}

fn main() {
    day_0101();
    day_0102();
    day_0201();
    day_0202();
    day_0301();
    day_0302();
}
