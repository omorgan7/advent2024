use regex::Regex;
use std::vec::Vec;

fn part1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|x| {
            let (_, [x, y]) = x.extract();
            x.parse::<i64>().unwrap() * y.parse::<i64>().unwrap()
        })
        .sum()
}

fn find_string_indexes(search: &[char], chars: &[char]) -> Vec<usize> {
    let mut i: usize = 0;
    let mut matches = Vec::new();

    'outer: loop {
        let mut success = false;
        for j in 0..search.len() {
            if i + j >= chars.len() {
                break 'outer;
            }

            if chars[i + j] != search[j] {
                break;
            }

            if j == search.len() - 1 {
                success = true;
                matches.push(i);
                i += search.len();
            }
        }

        if !success {
            i += 1;
        }

        if i >= chars.len() {
            break;
        }
    }

    matches
}

fn to_parsed_digit(digits: &[char]) -> i64 {
    let mut acc = 0;
    let mut base = 1;

    for x in digits.iter().rev() {
        acc += base * (x.to_digit(10).unwrap() as i64);
        base *= 10;
    }

    acc
}

fn part2(input: &str) -> i64 {
    let chars = input.chars().collect::<Vec<char>>();
    let mut dos = find_string_indexes(&['d', 'o', '(', ')'], &chars);
    let mut donts = find_string_indexes(&['d', 'o', 'n', '\'', 't', '(', ')'], &chars);

    let mut accumulator: i64 = 0;

    let mut state = true;

    let mut parsing_mul = false;
    let mul_first_half = ['m', 'u', 'l', '('];

    let mut left_digit = Vec::new();
    let mut right_digit = Vec::new();

    dos.insert(0, 0);
    dos.reverse();
    donts.reverse();

    let mut i: usize = 0;
    'outer: loop {
        let last_do = dos.last();
        let last_dont = donts.last();

        if let Some(x) = last_do {
            if let Some(y) = last_dont {
                if x < y && i >= *x {
                    state = true;
                    dos.pop();
                } else if y < x && i >= *y {
                    state = false;
                    donts.pop();
                }
            } else if i >= *x {
                state = true;
            }
        } else if !donts.is_empty() && i >= *last_dont.unwrap() {
            state = false;
        }

        if !parsing_mul {
            for j in 0..mul_first_half.len() {
                if i + j >= chars.len() {
                    break 'outer;
                }

                if chars[i + j] != mul_first_half[j] {
                    parsing_mul = false;
                    break;
                }

                parsing_mul = true;

                if j == mul_first_half.len() - 1 {
                    i += mul_first_half.len();
                }
            }
        }
        if !parsing_mul {
            i += 1;
            continue;
        }

        parsing_mul = false;

        left_digit.clear();
        right_digit.clear();

        let mut j = 0;
        loop {
            if i + j >= chars.len() {
                break 'outer;
            }
            if !chars[i + j].is_ascii_digit() {
                break;
            }

            left_digit.push(chars[i + j]);
            j += 1
        }

        i += j;

        if left_digit.is_empty() {
            continue;
        }

        if i >= chars.len() {
            break;
        }
        let is_comma = chars[i] == ',';
        i += 1;
        if !is_comma {
            continue;
        }

        let mut j = 0;
        loop {
            if i + j >= chars.len() {
                break 'outer;
            }

            if !chars[i + j].is_ascii_digit() {
                break;
            }

            right_digit.push(chars[i + j]);
            j += 1
        }

        i += j;
        if right_digit.is_empty() {
            continue;
        }

        if i >= chars.len() {
            break;
        }
        let is_closing = chars[i] == ')';
        i += 1;
        if !is_closing {
            continue;
        }

        if state {
            accumulator += to_parsed_digit(&left_digit) * to_parsed_digit(&right_digit);
        }

        if i >= chars.len() {
            break;
        }
    }

    accumulator
}

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part1(input));
    println!("{}", part2(input));
}
