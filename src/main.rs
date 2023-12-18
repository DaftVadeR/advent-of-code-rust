//use chrono;
//use std::env;
//use std::fs;

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

const NUMBERS_TO_WORDS: [&'static str; 10] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

type CodeNumber = (u32, usize);
//struct CodeNumber(digit: u32, index: usize);

fn get_first_number(line: &str) -> CodeNumber {
    

    let a: CodeNumber = (1,1);
    a
}

fn get_last_number() {
    
}

fn parse_words() -> Vec<u32> {
    let first_number_str: Option<u32> = None;
    let second_number_str: Option<u32> = None;

    let earliest_word_number: Option<usize> = None;
    let latest_word_number: Option<usize> = None;

    for word in NUMBERS_TO_WORDS {
        let temp_first_number_pos = line.find(word);
        let temp_second_number_pos = line.rfind(word);

        first_number_str = match temp_first_number_pos {
            Some(val) => {
                if(earliest_word_number.unwrap_or(-1) )
                val
            },
            None => None,
        }
    }


}

fn parse_digits(line: &str) -> Vec<u32> {
    line 
        .chars()
        .filter_map(|num_char| num_char.to_digit(10))
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

type SelectedCodeNumber = (Option<CodeNumber>, Option<CodeNumber>);

fn get_first_last_numbers(line: &str) -> Option<SelectedCodeNumber> {
    let digits = parse_digits(&line);
     
    if digits.is_empty() {
        None
        // return ((CodeNumber(None), CodeNumber(None));
    }
    // debug!(digits);
    
    // println!("{}", line);
    // println!("{}", digits.len());

    // let first_number: u32 = digits.first().unwrap_or(None)
    // let last_number = digits.last().unwrap_or(first_number);
    // 
    // let first: Option<CodeNumber> =  

    let a: u32 = 13;
    let b: usize = 14;
    ((a, b) as SelectedCodeNumber)
}

fn get_file_reader() -> BufReader<File> {
    let file = File::open("d1.txt").expect("Can't open file d1.txt");
    let reader = BufReader::new(file);

    assert!(reader.buffer().is_empty());

    reader
}

fn get_numbers_from_file() -> (Vec<String>, u32) {
    let mut final_numbers = vec![];
    let mut total = 0;
    
    let reader = get_file_reader();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        let first_last_numbers = get_first_last_numbers();

        // let concatenated = format!("{}{}", first_number, last_number);

        // total+= concatenated.parse::<u32>().unwrap();  

        // final_numbers.push(concatenated);
    }

   // (final_numbers, total)
}

#[cfg(test)]
mod tests {
    use crate::get_numbers_from_file;

    #[test]
    fn it_works() {
        let (numbers, total) = get_numbers_from_file();
        assert_eq!(numbers[0], "85");
        assert_eq!(numbers[1], "22");
        assert_eq!(numbers[2], "66");
        assert_eq!(numbers[3], "72");
        assert_eq!(numbers[4], "84");
        assert_eq!(numbers[5], "98");

        // total of all numbers
        println!("Total: {}", total);
    }
}

