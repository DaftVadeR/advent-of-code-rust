// use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};

const NUMBERS_TO_WORDS: [&'static str; 10] = [
    "zero", // Might cause issues - remove if so
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_words(line: &str) -> Vec<CodeNumber> {
    let mut numbers: Vec<CodeNumber> = vec![];

    // Index used as word value
    for (index, word) in NUMBERS_TO_WORDS.iter().enumerate() {
        let found_words: Vec<_> = line.match_indices(word).collect();

        if found_words.is_empty() {
            continue;
        }

        for (pos, _) in found_words {
            let digit_from_index = index.try_into();

            match digit_from_index {
                Ok(digit) => numbers.push(CodeNumber { digit, index: pos }),
                _ => (),
            };
        }
    }

    numbers
}

fn parse_digits(line: &str) -> Vec<CodeNumber> {
    let mut digits: Vec<CodeNumber> = vec![];
    // Used due to limitations of map + filter.
    for (index, ch) in line.chars().enumerate() {
        let num_opt = ch.to_digit(10);
        if num_opt.is_some() {
            let num = num_opt.unwrap();
            let code_number = CodeNumber { digit: num, index };

            digits.push(code_number);
        }
    }

    digits
}

fn main() {
    let (numbers, total) = get_numbers_from_file();

    for num in numbers {
        println!("Number: {}", num.final_str);
    }

    println!("---");
    println!("---");
    println!("---");
    println!("---");

    println!("Total: {}", total);
}

#[derive(Clone, Debug)]
struct CodeNumber {
    digit: u32,
    index: usize,
}

#[derive(Clone, Debug)]
struct LineNumbers {
    first: CodeNumber,
    last: CodeNumber,
    final_str: String,
}

fn get_first_and_last(digits: &Vec<CodeNumber>) -> Option<LineNumbers> {
    if digits.is_empty() {
        return None;
    }

    let mut temp_first = &digits[0];
    let mut temp_last = &digits[0];

    for codenum in digits {
        println!("{}{}", codenum.digit, codenum.index);

        if codenum.digit == 0 {
            // Will get a 0 if "zero" is in the example file - not a part of the question.
            continue;
        }

        if codenum.index < temp_first.index {
            temp_first = codenum;
        }

        if codenum.index > temp_last.index {
            temp_last = codenum;
        }
    }

    Some(LineNumbers {
        first: temp_first.clone(),
        last: temp_last.clone(),
        final_str: format!("{}{}", temp_first.digit, temp_last.digit),
    })
}

fn get_file_reader() -> BufReader<File> {
    let file = File::open("d1.txt").expect("Can't open file d1.txt");
    let reader = BufReader::new(file);

    assert!(reader.buffer().is_empty());

    reader
}

fn get_numbers_from_file() -> (Vec<LineNumbers>, u32) {
    let mut final_numbers: Vec<LineNumbers> = vec![];
    let mut total = 0;

    let reader = get_file_reader();
    let reader2 = get_file_reader();
    let count = reader2.lines().count();

    println!("Lines in file: {}", count);

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            break; // End of file
        }

        let mut from_numbers = parse_digits(&line);
        let mut from_words = parse_words(&line);

        let mut compare_indexes: Vec<CodeNumber> = vec![];

        compare_indexes.append(&mut from_numbers);
        compare_indexes.append(&mut from_words);

        let first_last_numbers = get_first_and_last(&compare_indexes);

        if first_last_numbers.is_none() {
            continue;
        }

        let line_numbers = first_last_numbers.unwrap();

        total += line_numbers.final_str.parse::<u32>().unwrap();

        final_numbers.push(line_numbers);
    }

    (final_numbers, total)
}

#[cfg(test)]
mod tests {
    use crate::get_numbers_from_file;

    #[test]
    fn it_works() {
        let (numbers, total) = get_numbers_from_file();
        assert_eq!(numbers[0].final_str, "85");
        assert_eq!(numbers[1].final_str, "22");
        assert_eq!(numbers[2].final_str, "66");
        assert_eq!(numbers[3].final_str, "72");
        assert_eq!(numbers[4].final_str, "84");
        assert_eq!(numbers[5].final_str, "98");
        assert_eq!(numbers[39].final_str, "99");

        // THIS IS THE PROBLEM CHILD
        assert_eq!(numbers[43].final_str, "22");

        assert_eq!(numbers[42].final_str, "88");
        assert_eq!(numbers[52].final_str, "27");
        assert_eq!(numbers[74].final_str, "22");

        for num in numbers {
            assert!(num.final_str.len() == 2);
        }

        // total of all numbers
        println!("Total: {}", total);
    }
}
