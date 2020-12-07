use std::io::{stdin, BufRead};
use std::convert::TryFrom;

fn main() {
    // <start>-<end> <character>: <string>
    let mut trees_1 = 0;
    let mut trees_2 = 0;
    let mut trees_3 = 0;
    let mut trees_4 = 0;
    let mut trees_5 = 0;
    let mut next_position_1 = 0;
    let mut next_position_2 = 0;
    let mut next_position_3 = 0;
    let mut next_position_4 = 0;
    let mut next_position_5 = 0;
    let mut read_position_5 = true;
    for line_result in stdin().lock().lines() {
        let line = line_result.unwrap();
        let length = line.chars().count();

        // Right 1, down 1.
        let pos_1 = line.chars().nth(next_position_1).unwrap();
        if pos_1 == '#' {
            trees_1 = trees_1 + 1;
        }
        next_position_1 = (next_position_1 + 1) % length;

        // Right 3, down 1. (This is the slope you already checked.)
        let pos_2 = line.chars().nth(next_position_2).unwrap();
        if pos_2 == '#' {
            trees_2 = trees_2 + 1;
        }
        next_position_2 = (next_position_2 + 3) % length;

        // Right 5, down 1.
        let pos_3 = line.chars().nth(next_position_3).unwrap();
        if pos_3 == '#' {
            trees_3 = trees_3 + 1;
        }
        next_position_3 = (next_position_3 + 5) % length;

        // Right 7, down 1.
        let pos_4 = line.chars().nth(next_position_4).unwrap();
        if pos_4 == '#' {
            trees_4 = trees_4 + 1;
        }
        next_position_4 = (next_position_4 + 7) % length;

        // Right 1, down 2.
        if read_position_5 {
            let pos_5 = line.chars().nth(next_position_5).unwrap();
            if pos_5 == '#' {
                trees_5 = trees_5 + 1;
            }
            next_position_5 = (next_position_5 + 1) % length;
        }
        read_position_5 = !read_position_5;
    }
    println!("{} {} {} {} {}", trees_1, trees_2, trees_3, trees_4, trees_5);
    println!("{}", trees_1 * trees_2 * trees_3 * trees_4 * trees_5);

    // 2224913600 - calculated manually :D overflowed
}
