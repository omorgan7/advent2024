use std::collections::HashMap;

fn pt1_recurse(pattern: &[char], towels: &[Vec<char>]) -> bool {
    if pattern.is_empty() {
        return true;
    }

    for towel in towels {
        if towel.len() > pattern.len() {
            continue;
        }

        if pattern[0..towel.len()] == *towel && pt1_recurse(&pattern[towel.len()..], towels) {
            return true;
        }
    }

    false
}

fn pt2_recurse(
    pattern: &[char],
    towels: &[Vec<char>],
    mut count: i64,
    cache: &mut HashMap<Vec<char>, i64>,
) -> i64 {
    if pattern.is_empty() {
        return count + 1;
    }

    for towel in towels {
        if towel.len() > pattern.len() {
            continue;
        }

        if pattern[0..towel.len()] == *towel {
            if cache.contains_key(&pattern[towel.len()..].to_vec()) {
                count += *cache.get(&pattern[towel.len()..].to_vec()).unwrap();
            } else {
                let delta = pt2_recurse(&pattern[towel.len()..], towels, count, cache) - count;
                cache.insert(pattern[towel.len()..].to_vec(), delta);
                count += delta;
            }
        }
    }

    count
}

fn part1(towels: &[Vec<char>], patterns: &[Vec<char>]) -> i64 {
    patterns
        .iter()
        .filter(|pattern| pt1_recurse(pattern, towels))
        .count() as i64
}

fn part2(towels: &[Vec<char>], patterns: &[Vec<char>]) -> i64 {
    patterns
        .iter()
        .map(|pattern| {
            let mut cache = HashMap::new();
            pt2_recurse(pattern, towels, 0, &mut cache)
        })
        .sum::<i64>()
}

fn main() {
    let input = include_str!("../input.txt");

    let towels = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|t| t.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();

    let patterns = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{}", part1(&towels, &patterns));
    println!("{}", part2(&towels, &patterns));
}
