use std::io::BufRead;
use std::vec;

fn main() {
    let mut numbers: Vec<i32> = vec![];

    let stdin = std::io::stdin();
    let lock = stdin.lock();
    for line in lock.lines() {
        let input_number: i32 = line.unwrap().trim().parse().ok().expect("only numbers");
        if input_number <= 2020 {
            numbers.push(input_number);
        }
    }

    let mut first = 0;
    let mut second = 1;
    let mut third = 2;
    'outer: loop {
        if first >= numbers.len() {
            break;
        }
        loop {
            if second >= numbers.len() {
                break;
            }
            // if sum of first two numbers is already greater than 2020
            // ignore and move ahead
            if numbers[first] + numbers[second] > 2020 {
                second = second + 1;
                third = second + 1;
                continue;
            }
            loop {
                if third >= numbers.len() {
                    break;
                }
                if numbers[first] + numbers[second] + numbers[third] == 2020 {
                    break 'outer;
                }
                third = third + 1;
            }
            second = second + 1;
            third = second + 1;
        }
        first = first + 1;
        second = first + 1;
        third = second + 1;
    }
    println!("{}", numbers[first] * numbers[second] * numbers[third]);
}
