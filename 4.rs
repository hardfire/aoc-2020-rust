use std::io::{stdin, BufRead};

fn main() {
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;
    let mut cid = false;

    let mut valid_passports = 0;

    for line_result in stdin().lock().lines() {
        let line = line_result.unwrap();
        let length = line.chars().count();
        if length == 0 {
            if byr && iyr && eyr && hgt && hcl && ecl && pid {
                valid_passports = valid_passports + 1;
                println!("valid {} {} {} {} {} {} {} {}", byr, iyr , eyr , hgt , hcl , ecl , pid , cid);
            } else {
                println!("invalid {} {} {} {} {} {} {} {}", byr, iyr , eyr , hgt , hcl , ecl , pid , cid);
            }
            byr = false;
            iyr = false;
            eyr = false;
            hgt = false;
            hcl = false;
            ecl = false;
            pid = false;
            cid = false;
            continue;
        }

        let split = line.split(" ");
        for item in split {
            let item_key = item.split(":").next().unwrap().parse::<String>().unwrap();
            match item_key.as_str() {
                "byr" =>  byr = true,
                "iyr" =>  iyr = true,
                "eyr" =>  eyr = true,
                "hgt" =>  hgt = true,
                "hcl" =>  hcl = true,
                "ecl" =>  ecl = true,
                "pid" =>  pid = true,
                "cid" =>  cid = true,
                _ => println!("something else!"),
            }
        }
    }
    if byr && iyr && eyr && hgt && hcl && ecl && pid {
        valid_passports = valid_passports + 1
    }

    println!("{}", valid_passports);
}
