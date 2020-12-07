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

    let mut start = 0;
    let mut next = 1;
    'outer: loop {
        if start >= numbers.len() {
            break;
        }
        loop {
            if next >= numbers.len() {
                break;
            }
            if numbers[start] + numbers[next] == 2020 {
                break 'outer
            }
            next = next + 1;
        }
        start = start+1 ;
        next = start+1;
    }
    println!("{}", numbers[start]*numbers[next]);
}
