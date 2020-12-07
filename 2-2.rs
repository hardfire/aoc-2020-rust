use std::io::{stdin, BufRead};
use std::convert::TryFrom;

fn main() {
    // <start>-<end> <character>: <string>
    let mut valid_password = 0;
    for line_result in stdin().lock().lines() {
        let line = line_result.unwrap();
        let mut split = line.split("-");
        let start:i32 = split.next().unwrap().parse::<i32>().unwrap();

        split = split.next().unwrap().split(": ");
        let mut end_char= split.next().unwrap().split(" ");

        let end:i32 = end_char.next().unwrap().parse::<i32>().unwrap();
        let required:char= end_char.next().unwrap().parse::<char>().unwrap();
        let password = split.next().unwrap();

        let start_index = usize::try_from(start - 1).unwrap();
        let end_index = usize::try_from(end- 1).unwrap();

        let is_start = password.chars().nth(start_index).unwrap() == required;
        let is_end = password.chars().nth(end_index).unwrap() == required;

        if is_start ^ is_end {
            println!("valid {} {} {} {}", password, start, end, required);
            valid_password = valid_password + 1;
            continue;
        }
        println!("invalid {} {} {} {}", password, start, end, required);
    }
    println!("{}", valid_password);
}
