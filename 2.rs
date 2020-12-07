use std::io::{stdin, BufRead};

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

        let mut count = 0;
        for character in password.chars() {
            if character == required{
                count = count + 1
            }
            if count > end{
                break;
            }
        }
        if count >= start && count <= end {
            valid_password = valid_password + 1;
            println!("valid: {} {} {}", password, required, count);
        } else {
            println!("invalid: {} {} {}", password, required, count);
        }
    }
    println!("{}", valid_password);
}
