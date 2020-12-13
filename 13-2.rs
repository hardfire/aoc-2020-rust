use std::io::{self, BufRead};
use std::vec;

fn main() {
    let mut t: i64 = 0;
    let mut period: i64 = 0;
    let mut timings: Vec<(i64, i64)> = vec![];

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    iterator.next(); // just ignore 

    let buses = iterator.next().unwrap().unwrap();
    let mut time_difference = 0;
    for bus in buses.split(",") {
        // out of service
        if bus == "x" {
            time_difference += 1;
            continue;
        }

        let bus_id: i64 = bus.parse().ok().expect("bus number is not a number");
        timings.push((bus_id, time_difference));

        // first entry
        if t == 0 {
            t = bus_id;
            period = bus_id;
        }  else {
            loop {
                if (t + time_difference) % bus_id != 0 {
                    t += period
                } else {
                    period = period * bus_id;
                    break;
                }
            }
        }
        time_difference += 1;
    }
    println!("{}", t);
}
