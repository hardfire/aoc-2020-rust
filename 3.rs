use std::convert::TryFrom;
use std::io::{stdin, BufRead};

fn main() {
    // <start>-<end> <character>: <string>
    let mut trees = 0;
    let mut next_position = 0;
    for line_result in stdin().lock().lines() {
        let line = line_result.unwrap();
        let length = line.chars().count();
        let pos = line.chars().nth(next_position).unwrap();
        if pos == '#' {
            trees = trees + 1;
        }
        next_position = (next_position + 3) % length;
        println!("{} {} {}", pos, length, next_position);
    }
    println!("{}", trees);
}
