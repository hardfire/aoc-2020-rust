use std::io::{stdin, BufRead};

fn main() {
    let mut count: [i32; 26] = [0; 26];
    let mut total_people = 0;
    let mut sum = 0;

    for line_result in stdin().lock().lines() {
        let line = line_result.unwrap();
        if line.chars().count() == 0 {
            let total = count.iter().filter(|&a| *a == total_people).count();
            sum += total;
            count = [0; 26];
            total_people = 0;
            continue;
            // reset array to all empty
        }
        for character in line.chars() {
            let val = character as u32 - 97;
            count[val as usize] += 1;
        }
        total_people += 1;
    }
    let total = count.iter().filter(|&a| *a == total_people).count();
    sum += total;

    println!("{}", sum);
}
