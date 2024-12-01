use std::collections::HashMap;
use std::vec::Vec;

fn pt1(lhs: &[i64], rhs: &[i64]) -> i64 {
    let mut lhs_sorted = lhs.to_vec();
    lhs_sorted.sort();

    let mut rhs_sorted = rhs.to_vec();
    rhs_sorted.sort();

    lhs_sorted
        .into_iter()
        .zip(rhs_sorted)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn pt2(lhs: &[i64], rhs: &[i64]) -> i64 {
    let mut counts = HashMap::new();
    for c in rhs {
        let entry = counts.entry(c).or_insert(0);
        *entry += 1;
    }

    lhs.iter().map(|l| l * counts.get(l).unwrap_or(&0)).sum()
}

fn main() {
    let input = include_str!("../input.txt");

    let (lhs, rhs): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut split = line.split("   ");
            let lhs = split.next().unwrap().parse::<i64>().unwrap();
            let rhs = split.next().unwrap().parse::<i64>().unwrap();

            (lhs, rhs)
        })
        .unzip();

    println!("{}", pt1(&lhs, &rhs));
    println!("{}", pt2(&lhs, &rhs));
}
