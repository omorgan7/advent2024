use multimap::MultiMap;
use std::collections::HashSet;
use std::vec::Vec;

struct Grid {
    grid: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        let mut grid: Vec<_> = input.chars().collect();
        grid.retain(|x| *x != '\n');

        Grid {
            grid,
            width,
            height,
        }
    }

    fn out_of_bounds(&self, x: i64, y: i64) -> bool {
        if x < 0 || x >= self.width as i64 {
            return true;
        }
        if y < 0 || y >= self.height as i64 {
            return true;
        }

        false
    }

    fn at_unchecked(&self, x: i64, y: i64) -> char {
        self.grid[(x as usize) + self.width * (y as usize)]
    }
}

fn add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

fn subtract(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 - b.0, a.1 - b.1)
}

fn flip(a: (i64, i64)) -> (i64, i64) {
    (-a.0, -a.1)
}

fn part1(grid: &Grid) -> i64 {
    let mut antennas = MultiMap::<char, (i64, i64)>::new();

    for y in 0..grid.height as i64 {
        for x in 0..grid.width as i64 {
            let g = grid.at_unchecked(x, y);
            if g != '.' {
                antennas.insert(g, (x, y));
            }
        }
    }

    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();

    for (_k, locations) in antennas.iter_all() {
        for i in 0..locations.len() {
            let x = locations[i];
            for j in (i + 1)..locations.len() {
                let y = locations[j];

                let direction = subtract(x, y);

                let l0 = add(x, direction);
                let l1 = add(y, flip(direction));

                if !grid.out_of_bounds(l0.0, l0.1) {
                    antinodes.insert(l0);
                }

                if !grid.out_of_bounds(l1.0, l1.1) {
                    antinodes.insert(l1);
                }
            }
        }
    }

    antinodes.len() as i64
}

fn part2(grid: &Grid) -> i64 {
    let mut antennas = MultiMap::<char, (i64, i64)>::new();

    for y in 0..grid.height as i64 {
        for x in 0..grid.width as i64 {
            let g = grid.at_unchecked(x, y);
            if g != '.' {
                antennas.insert(g, (x, y));
            }
        }
    }

    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();

    for (_k, locations) in antennas.iter_all() {
        for i in 0..locations.len() {
            let x = locations[i];
            for j in (i + 1)..locations.len() {
                let y = locations[j];

                let direction = subtract(x, y);
                let flip_direction = flip(direction);

                let mut positive = true;
                let mut negative = true;

                let mut l0 = y;
                let mut l1 = x;

                while positive || negative {
                    if positive {
                        l0 = add(l0, direction);
                        if !grid.out_of_bounds(l0.0, l0.1) {
                            antinodes.insert(l0);
                        } else {
                            positive = false;
                        }
                    }
                    if negative {
                        l1 = add(l1, flip_direction);
                        if !grid.out_of_bounds(l1.0, l1.1) {
                            antinodes.insert(l1);
                        } else {
                            negative = false;
                        }
                    }
                }
            }
        }
    }

    antinodes.len() as i64
}

fn main() {
    let grid = Grid::new(include_str!("../input.txt"));
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}
