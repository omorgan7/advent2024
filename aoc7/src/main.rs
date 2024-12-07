use std::vec::Vec;

fn recurse_operator_check_pt1(target: i64, accumulator: i64, numbers: &[i64]) -> bool {
    if target == accumulator && numbers.is_empty() {
        return true;
    }
    if numbers.is_empty() {
        return false;
    }
    let add_accumulator = accumulator + numbers[0];
    if recurse_operator_check_pt1(target, add_accumulator, &numbers[1..]) {
        return true;
    }

    recurse_operator_check_pt1(target, accumulator * numbers[0], &numbers[1..])
}

fn recurse_operator_check_pt2(target: i64, accumulator: i64, numbers: &[i64]) -> bool {
    if target == accumulator && numbers.is_empty() {
        return true;
    }
    if numbers.is_empty() {
        return false;
    }

    if recurse_operator_check_pt2(target, accumulator + numbers[0], &numbers[1..]) {
        return true;
    }

    if recurse_operator_check_pt2(target, accumulator * numbers[0], &numbers[1..]) {
        return true;
    }

    let concated = (accumulator.to_string() + &numbers[0].to_string())
        .parse::<i64>()
        .unwrap();
    recurse_operator_check_pt2(target, concated, &numbers[1..])
}

fn part1(targets: &[i64], all_numbers: &[Vec<i64>]) -> i64 {
    targets
        .iter()
        .zip(all_numbers)
        .filter_map(|(target, numbers)| {
            if recurse_operator_check_pt1(*target, numbers[0], &numbers[1..]) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

fn part2(targets: &[i64], all_numbers: &[Vec<i64>]) -> i64 {
    targets
        .iter()
        .zip(all_numbers)
        .filter_map(|(target, numbers)| {
            if recurse_operator_check_pt2(*target, numbers[0], &numbers[1..]) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");

    let (targets, numbers): (Vec<i64>, Vec<Vec<i64>>) = input
        .lines()
        .map(|line| {
            let mut it = line.split(": ");
            let target = it.next().unwrap().parse::<i64>().unwrap();

            let numbers = it
                .next()
                .unwrap()
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            (target, numbers)
        })
        .unzip();

    println!("{}", part1(&targets, &numbers));
    println!("{}", part2(&targets, &numbers));
}
