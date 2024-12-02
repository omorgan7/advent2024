use std::vec::Vec;

fn check_level(level: &[i64]) -> bool
{
    let mut inc = false;
    let mut dec = false;

    for slice in level.windows(2) {
        if slice[0] == slice[1] {
            return false;
        }

        if slice[0] > slice[1] {
            if inc {
                return false;
            }
            if slice[0] - slice[1] > 3 {
                return false;
            }
            dec = true;
        }

        if slice[0] < slice[1] {
            if dec {
                return false;
            }
            
            if slice[1] - slice[0] > 3 {
                return false;
            }

            inc = true;
        }
    }

    true
}

fn part1(levels: &Vec<Vec<i64>>) -> i64
{
    levels.iter().filter(|l| check_level(l)).count() as i64
}

fn part2(levels: &Vec<Vec<i64>>) -> i64
{
    levels.iter().filter(|level| {
        if check_level(level) {
            return true;
        }

        (0..level.len()).any(|i| {
            let mut copied = level.to_vec();
            copied.remove(i);

            check_level(&copied)
        })

    }).count() as i64
}

fn main() {
    let input = include_str!("../input.txt");

    let levels : Vec<_> =  input.lines().map(|line| {
        line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>()
    }).collect();

    println!("{}", part1(&levels));
    println!("{}", part2(&levels));
}
