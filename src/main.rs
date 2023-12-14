use chrono;
use std::env;
use std::fs;

use std::fs::File;
use std::io::{BufReader, BufRead};

// let time_date = chrono::NaiveTime::parse_from_str(release.time.as_str(), "%H:%M")
// .expect("Could not parse time");

// now = now
//     .with_hour(time_date.hour())
//     .expect("Couldn't change hour");
// now = now
//     .with_minute(time_date.minute())
//     .expect("Couldn't change minute");
//
// let current_date_str = now.format("%Y-%m-%d %H:%M").to_string();

fn parse_digits(t_num: &str) -> Vec<u32> {
    t_num
        .chars()
        .filter_map(|a| a.to_digit(10))
        .collect()
}

fn main() {
    // let contents =
    //     fs::read_to_string("d1.txt".to_string()).expect("Should have been able to read the file");
    // println!("With text:\n{contents}");
    // for line in contents {
    //     
    // }

    let (numbers, total) = get_numbers_from_file();

    for num in numbers {
        println!("{}", num)
    }

    println!("{}", total);
}

fn get_numbers_from_file() -> (Vec<String>, u32) {
    let file = File::open("d1.txt").expect("Can't open file d1.txt");
    let reader = BufReader::new(file);

    let mut final_numbers = vec![];
    let mut total = 0;

    assert!(reader.buffer().is_empty());

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        let digits = parse_digits(&line);
        
        if digits.is_empty() {
            break;
        }
        // debug!(digits);
        
        println!("{}", line);
        println!("{}", digits.len());

        let first_number = digits.first().unwrap();
        let last_number = digits.last().unwrap_or(first_number);
        let concatenated = format!("{}{}", first_number, last_number);

        total+= concatenated.parse::<u32>().unwrap();  

        final_numbers.push(concatenated);
    }

    (final_numbers, total)
}

#[cfg(test)]
mod tests {
    use crate::get_numbers_from_file;

    #[test]
    fn it_works() {
        let (numbers, total) = get_numbers_from_file();
        assert_eq!(numbers[0], "45");
        assert_eq!(numbers[1], "22");
        assert_eq!(numbers[2], "66");
        assert_eq!(numbers[3], "22");
        assert_eq!(numbers[4], "84");

        // total of all numbers
        println!("Total: {}", total);
    }
}

