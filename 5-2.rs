use std::io::{stdin, BufRead};

fn get_seat_id(seat: String) -> i32 {
    let mut row = (0, 127);
    let mut col = (0, 7);

    for character in seat.chars() {
        match character {
            'B' => row.0 = (row.0 + row.1 + 1) / 2,
            'F' => row.1 = ((row.0 + row. 1 + 1) / 2) - 1,
            'R' => col.0 = (col.0 + col.1 + 1) / 2,
            'L' => col.1 = ((col.0 + col.1 + 1) / 2) - 1,
            _ => {} //igore this
        }
    }

    if row.0 == row. 1 && col.0 == col.1 {
        return (row.0 * 8) + col.0;
    } else {
        println!("something went wrong");
        return 0;
    }
}
fn main() {
    let mut seats: [i32; 954] = [0; 954];

    for line_result in stdin().lock().lines() {
        let line = line_result.unwrap();
        let seat_id = get_seat_id(line);
        seats[seat_id as usize] = 1
    }

    let window = seats.windows(3);
    for (i, next) in window.enumerate() {
        if next == &[1,0,1] {
            println!("{}", i + 1);
            break;
        }
    }

}
