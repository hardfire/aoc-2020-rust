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
            let mut keyval = item.split(":");
            let item_key = keyval.next().unwrap().parse::<String>().unwrap();
            match item_key.as_str() {
                "byr" =>  {
                    let val:i32 = keyval.next().unwrap().parse::<i32>().unwrap();
                    if val >= 1920 && val <= 2002 {
                        byr = true;
                    }
                },
                "iyr" =>  {
                    let val:i32 = keyval.next().unwrap().parse::<i32>().unwrap();
                    if val >= 2010 && val <= 2020 {
                        iyr = true;
                    }
                },
                "eyr" =>  {
                    let val:i32 = keyval.next().unwrap().parse::<i32>().unwrap();
                    if val >= 2020 && val <= 2030 {
                        eyr = true
                    }
                },
                "hgt" =>  {
                    /*
                     * a number followed by either cm or in:
                     * If cm, the number must be at least 150 and at most 193.
                     * If in, the number must be at least 59 and at most 76.
                     */
                    let val = keyval.next().unwrap();
                    let mut value : u32= 0;
                    let mut unit = String::new();
                    if val.chars().count() > 5 {
                        hgt = false
                    } else {
                        for character in val.chars() {
                            if character.is_digit(10) {
                                value = ( value * 10 ) + character.to_digit(10).unwrap();
                            } else {
                                unit.push(character);
                            }
                        }
                        match unit.as_str() {
                            "cm" =>  {
                                if value >=150 && value <= 193 {
                                    hgt = true;
                                }
                            },
                            "in" => {
                                if value >=59 && value <= 76 {
                                    hgt = true;
                                }
                            },
                            _ => {
                                println!("invalid unit");
                            }
                        }
                    }
                },
                "hcl" =>  {
                    let val = keyval.next().unwrap();
                    let mut first = true;
                    if val.chars().nth(0).unwrap() == '#' && val.chars().count() == 7 {
                        for character in val.chars() {
                            if first {
                                if character != '#'{
                                    hcl = false;
                                    break;
                                }
                                first = false;
                            }
                            else if !character.is_digit(16) {
                                hcl = false;
                                break;
                            }
                            hcl = true;
                        }
                    }
                },
                "ecl" => {
                    let val = keyval.next().unwrap();
                    ecl = match val{
                        "amb"|"blu"|"brn"|"gry"|"grn"|"hzl"|"oth" => true,
                        _ => false
                    }
                },
                "pid" =>  {
                    let val = keyval.next().unwrap();
                    if val.chars().count() == 9 {
                        for character in val.chars() {
                            if character.is_digit(10) {
                                pid = true;
                            } else {
                                pid = false;
                                break;
                            }
                        }
                    }
                },
                "cid" => cid = true,
                _ => println!("something else!"),
            }
        }
    }
    if byr && iyr && eyr && hgt && hcl && ecl && pid {
        valid_passports = valid_passports + 1
    }

    println!("{}", valid_passports);
}
