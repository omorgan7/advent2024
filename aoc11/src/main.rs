use either::Either;
use std::collections::HashMap;
use std::collections::VecDeque;

fn split_even_digits(y: i64) -> Option<[i64; 2]> {
    let yf = y as f64;

    let logbase = yf.log10().floor() as i64;
    if logbase % 2 != 1 {
        None
    } else {
        let halfbase = logbase / 2;

        let top = 10_i64.pow((halfbase + 1) as u32);
        let top_original = y / top;
        let bottom = y - (top_original * top);

        Some([top_original, bottom])
    }
}

fn next_number(x: i64) -> Either<i64, [i64; 2]> {
    if x == 0 {
        Either::Left(1)
    } else if let Some(digits) = split_even_digits(x) {
        Either::Right(digits)
    } else {
        Either::Left(x * 2024)
    }
}

fn part1(stones: &[i64]) -> i64 {
    stones
        .iter()
        .map(|stone| {
            let mut current_stones = VecDeque::new();
            current_stones.push_back(*stone);

            for _i in 0..25 {
                let mut next_stones = VecDeque::new();
                for stone in &current_stones {
                    let next = next_number(*stone);
                    match next {
                        Either::Left(y) => {
                            next_stones.push_back(y);
                        }
                        Either::Right(y) => {
                            next_stones.push_back(y[1]);
                            next_stones.push_back(y[0]);
                        }
                    }
                }

                current_stones = next_stones;
            }

            current_stones.len() as i64
        })
        .sum::<i64>()
}

const MAX_DEPTH: i64 = 75;

fn stone_count(
    stone: i64,
    mut count: i64,
    depth: i64,
    cache: &mut HashMap<(i64, i64), i64>,
) -> i64 {
    let next = next_number(stone);
    match next {
        Either::Left(y) => {
            if depth < MAX_DEPTH {
                if cache.contains_key(&(y, depth)) {
                    count += cache[&(y, depth)];
                } else {
                    let extra = stone_count(y, 1, depth + 1, cache) - 1;
                    count += extra;

                    cache.insert((y, depth), extra);
                }
            }
        }
        Either::Right(y) => {
            if depth < MAX_DEPTH {
                count =
                    stone_count(y[1], 1, depth + 1, cache) + stone_count(y[0], 1, depth + 1, cache);
            }
        }
    }

    count
}

fn part2(stones: &[i64]) -> i64 {
    let mut cache = HashMap::<(i64, i64), i64>::new();

    stones
        .iter()
        .map(|stone| stone_count(*stone, 1, 0, &mut cache))
        .sum::<i64>()
}

fn main() {
    let input = include_str!("../input.txt");

    let stones = input
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!("{}", part1(&stones));
    println!("{}", part2(&stones));
}
