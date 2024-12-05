use multimap::MultiMap;

fn solve(rules: &MultiMap<i64, i64>, inputs: &[Vec<i64>], is_pt1: bool) -> i64 {
    let empty = Vec::new();
    inputs
        .iter()
        .filter_map(|update| {
            let mut sorted = update.clone();
            sorted.sort_by(|a, b| {
                let relevant_rule = rules.get_vec(a).unwrap_or(&empty);
                if relevant_rule.contains(b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });

            if is_pt1 {
                if sorted == *update {
                    return Some(sorted[(sorted.len() - 1) / 2]);
                }
            } else if sorted != *update {
                return Some(sorted[(sorted.len() - 1) / 2]);
            }
            None
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");

    let mut input_it = input.split("\n\n");

    let rules = input_it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut it = line.split("|");
            let key = it.next().unwrap().parse::<i64>().unwrap();
            let val = it.next().unwrap().parse::<i64>().unwrap();

            (key, val)
        })
        .collect::<MultiMap<_, _>>();

    let inputs = input_it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<i64>>>();

    println!("{}", solve(&rules, &inputs, true));
    println!("{}", solve(&rules, &inputs, false));
}
