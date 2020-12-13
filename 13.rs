use std::io::{self, BufRead};

fn main() {
    let mut timestamp: i32 = 0;
    let mut previous_min : i32 = 0;
    let mut previous_min_bus : i32 = 0;

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    timestamp = iterator.next().unwrap().unwrap().parse().ok().expect("timestamp");
    previous_min = timestamp; // cannot be larger than this anyways

    let buses = iterator.next().unwrap().unwrap();

    for bus in buses.split(",") {
        // out of service
        if bus == "x" {
            continue;
        }

        let bus_id: i32 = bus.parse().ok().expect("bus number is not a number");
        let time_to_wait = timestamp % bus_id;

        // if we dont have to wait at all
        if time_to_wait == 0 {
            previous_min = 0;
            previous_min_bus = bus_id;
            break;
        }  else if (bus_id - time_to_wait) < previous_min {
            previous_min = bus_id - time_to_wait;
            previous_min_bus = bus_id;
        }
    }
    println!("{} {} - {}", previous_min_bus, previous_min, previous_min_bus * previous_min);

}
