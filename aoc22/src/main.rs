fn evolve_secret(mut secret: i64) -> i64 {
    let p1 = secret * 64;
    secret = (secret ^ p1) % 16777216;
    let p2 = secret / 32;
    secret = (secret ^ p2) % 16777216;
    let p3 = secret * 2048;
    secret = (secret ^ p3) % 16777216;

    secret
}

fn part1(input: &[i64]) -> i64
{
    input.iter().map(|x| {
        let mut secret = *x;
        for _ in 0..2000 {
            secret = evolve_secret(secret);
        }

        secret
    }).sum()
}

fn part2(input: &[i64]) -> i64
{
    let prices = input.iter().map(|x| {
        let mut secret = *x;

        let mut ps = Vec::new();
        ps.push(secret % 10);
        for _ in 0..2000 {
            secret = evolve_secret(secret);
            ps.push(secret % 10);
        }

        ps
    }).collect::<Vec<_>>();

    let diffs = prices.iter().map(|p| {
        let mut ds = Vec::new();

        for i in 1..p.len() {
            ds.push(p[i] - p[i-1]);
        }

        ds
    }).collect::<Vec<_>>();

    for (j, d ) in diffs.iter().enumerate() {
        for i in 0..d.len() {
            println!("{} ({})", prices[j][i + 1], d[i]);
        }
        println!();
    }

    prices.len() as i64
}

fn main() {
    let input = include_str!("../input.txt").lines().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<_>>();
    // println!("{}", part1(&input));
    println!("{}", part2(&input));
}
