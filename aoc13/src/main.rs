use regex::Regex;
use std::vec::Vec;

#[derive(Debug)]
struct Input {
    a: (i64, i64),
    b: (i64, i64),
    goal: (i64, i64),
}

fn part1(inputs: &[Input]) -> i64 {
    inputs
        .iter()
        .filter_map(|x| {
            for i in (1..=100).rev() {
                for j in 1..=100 {
                    if i * x.a.0 + j * x.b.0 == x.goal.0 && i * x.a.1 + j * x.b.1 == x.goal.1 {
                        return Some(3 * i + j);
                    }
                }
            }
            None
        })
        .sum::<i64>()
}

fn part2(inputs: &[Input]) -> i64 {
    inputs
        .iter()
        .filter_map(|x| {
            let goal = (10000000000000 + x.goal.0, 10000000000000 + x.goal.1);
            let lhs = x.a.1 * -x.b.0 + x.b.1 * x.a.0;
            let rhs = x.a.0 * goal.1 - x.a.1 * goal.0;

            if rhs % lhs != 0 {
                None
            } else {
                let b_press = rhs / lhs;
                if (goal.0 - b_press * x.b.0) % x.a.0 != 0 {
                    None
                } else {
                    let a_press = (goal.0 - b_press * x.b.0) / x.a.0;
                    Some(a_press * 3 + b_press)
                }
            }
        })
        .sum::<i64>()
}

fn main() {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .ok()
    .unwrap();

    let inputs = re
        .captures_iter(include_str!("../input.txt"))
        .map(|x| {
            let (_, numbers) = x.extract::<6>();
            let mut it = numbers.iter().map(|y| y.parse::<i64>().unwrap());

            Input {
                a: (it.next().unwrap(), it.next().unwrap()),
                b: (it.next().unwrap(), it.next().unwrap()),
                goal: (it.next().unwrap(), it.next().unwrap()),
            }
        })
        .collect::<Vec<Input>>();

    println!("{}", part1(&inputs));
    println!("{}", part2(&inputs));
}
