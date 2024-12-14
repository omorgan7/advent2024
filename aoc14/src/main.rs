use std::vec::Vec;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vector2 {
    x: i64,
    y: i64,
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone)]
struct Robot {
    position: Vector2,
    velocity: Vector2,
}

fn handle_bounds(mut a: Vector2, bounds: Vector2) -> Vector2 {
    if a.x >= bounds.x {
        a.x -= bounds.x;
    }

    if a.y >= bounds.y {
        a.y -= bounds.y;
    }

    while a.x < 0 {
        a.x += bounds.x;
    }

    while a.y < 0 {
        a.y += bounds.y;
    }

    a
}

fn part1(robots: &[Robot]) -> i64 {
    let mut robots = robots.to_vec();

    let bounds = Vector2 { x: 101, y: 103 };

    for robot in robots.iter_mut() {
        for _ in 0..100 {
            robot.position = handle_bounds(robot.position + robot.velocity, bounds);
        }
    }

    let midpoint = Vector2 {
        x: bounds.x / 2,
        y: bounds.y / 2,
    };

    let mut counts: [i64; 4] = [0; 4];

    for robot in robots {
        if robot.position.x < midpoint.x {
            if robot.position.y < midpoint.y {
                counts[0] += 1;
            } else if robot.position.y > midpoint.y {
                counts[2] += 1;
            }
        } else if robot.position.x > midpoint.x {
            if robot.position.y < midpoint.y {
                counts[1] += 1;
            } else if robot.position.y > midpoint.y {
                counts[3] += 1;
            }
        }
    }

    counts.iter().product()
}

fn part2(robots: &[Robot]) -> i64 {
    let mut robots = robots.to_vec();

    let bounds = Vector2 { x: 101, y: 103 };

    let iteration = 0;

    let midpoint = Vector2 {
        x: bounds.x / 2,
        y: bounds.y / 2,
    };
    let mut all_counts = Vec::new();

    for _ in 0..10000 {
        for robot in robots.iter_mut() {
            robot.position = handle_bounds(robot.position + robot.velocity, bounds);
        }

        let mut counts: [i64; 4] = [0; 4];

        for robot in &robots {
            if robot.position.x < midpoint.x {
                if robot.position.y < midpoint.y {
                    counts[0] += 1;
                } else if robot.position.y > midpoint.y {
                    counts[2] += 1;
                }
            } else if robot.position.x > midpoint.x {
                if robot.position.y < midpoint.y {
                    counts[1] += 1;
                } else if robot.position.y > midpoint.y {
                    counts[3] += 1;
                }
            }
        }

        all_counts.push(counts);
    }

    let mut count_maxes = [0; 4];

    for i in 0..all_counts.len() {
        for j in 0..4 {
            if all_counts[i][j] > all_counts[count_maxes[j]][j] {
                count_maxes[j] = i;
            }
        }
    }

    for j in 0..4 {
        println!(
            "Max count in quadratn {} was {:?} at iteration {}",
            j, all_counts[count_maxes[j]], count_maxes[j]
        );
    }

    iteration
}

fn main() {
    let input = include_str!("../input.txt");

    let robots = input
        .lines()
        .map(|line| {
            let mut it = line.split(" ");

            let mut lhs = it.next().unwrap()[2..]
                .split(",")
                .map(|x| x.parse::<i64>().unwrap());

            let mut rhs = it.next().unwrap()[2..]
                .split(",")
                .map(|x| x.parse::<i64>().unwrap());

            Robot {
                position: Vector2 {
                    x: lhs.next().unwrap(),
                    y: lhs.next().unwrap(),
                },
                velocity: Vector2 {
                    x: rhs.next().unwrap(),
                    y: rhs.next().unwrap(),
                },
            }
        })
        .collect::<Vec<Robot>>();

    println!("{}", part1(&robots));
    println!("{}", part2(&robots));
}
