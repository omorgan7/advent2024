use std::collections::HashSet;

struct Grid {
    grid: Vec<i64>,
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
            grid: grid
                .iter()
                .map(|x| x.to_digit(10).unwrap() as i64)
                .collect(),
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

    fn at(&self, x: i64, y: i64) -> Option<i64> {
        if self.out_of_bounds(x, y) {
            None
        } else {
            Some(self.grid[(x as usize) + self.width * (y as usize)])
        }
    }

    fn at_unchecked(&self, x: i64, y: i64) -> i64 {
        self.grid[(x as usize) + self.width * (y as usize)]
    }

    fn neighbours() -> [(i64, i64); 4] {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
    }
}

fn add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

fn part1(grid: &Grid) -> i64 {
    let mut stack = Vec::<((i64, i64), i64)>::new();

    let mut found = HashSet::new();

    for y in 0..grid.height as i64 {
        for x in 0..grid.width as i64 {
            stack.clear();
            if grid.at_unchecked(x, y) == 0 {
                stack.push(((x, y), 0));
            }

            while let Some(point) = stack.pop() {
                if grid.at(point.0 .0, point.0 .1).unwrap_or(-1) != point.1 {
                    continue;
                }

                if point.1 == 9 && !found.contains(&((x, y), point.0)) {
                    found.insert(((x, y), point.0));
                }

                let next = point.1 + 1;
                if next > 9 {
                    continue;
                }

                for n in Grid::neighbours() {
                    let neighbour = add(point.0, n);
                    if grid.at(neighbour.0, neighbour.1).unwrap_or(-1) == next {
                        stack.push((neighbour, next));
                    }
                }
            }
        }
    }

    found.len() as i64
}

fn part2(grid: &Grid) -> i64 {
    let mut stack = Vec::<((i64, i64), i64)>::new();

    let mut count = 0;

    for y in 0..grid.height as i64 {
        for x in 0..grid.width as i64 {
            stack.clear();
            if grid.at_unchecked(x, y) == 0 {
                stack.push(((x, y), 0));
            }

            while let Some(point) = stack.pop() {
                
                if grid.at(point.0 .0, point.0 .1).unwrap_or(-1) != point.1 {
                    continue;
                }

                if point.1 == 9 {
                    count += 1;
                }

                let next = point.1 + 1;
                if next > 9 {
                    continue;
                }

                for n in Grid::neighbours() {
                    let neighbour = add(point.0, n);
                    if grid.at(neighbour.0, neighbour.1).unwrap_or(-1) == next {
                        stack.push((neighbour, next));
                    }
                }
            }
        }
    }

    count as i64
}

fn main() {
    let input = include_str!("../input.txt");

    let grid = Grid::new(input);

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}
