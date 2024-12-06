use std::collections::HashSet;
use std::vec::Vec;

struct Grid {
    grid: Vec<char>,
    width: usize,
    height: usize,
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Grid {
            grid: self.grid.clone(),
            width: self.width,
            height: self.height,
        }
    }
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

    fn find_starting_position(&self) -> (i64, i64) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[x + self.width * y] == '^' {
                    return (x as i64, y as i64);
                }
            }
        }
        panic!()
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

    fn at(&self, x: i64, y: i64) -> Option<char> {
        if self.out_of_bounds(x, y) {
            None
        } else {
            Some(self.grid[(x as usize) + self.width * (y as usize)])
        }
    }

    fn at_mut(&mut self, x: i64, y: i64) -> &mut char {
        &mut self.grid[(x as usize) + self.width * (y as usize)]
    }
}

fn next_direction(direction: (i64, i64)) -> (i64, i64) {
    if direction.1 == -1 {
        return (1, 0);
    }
    if direction.0 == 1 {
        return (0, 1);
    }
    if direction.1 == 1 {
        return (-1, 0);
    }
    (0, -1)
}

fn detect_loop(grid: &Grid, starting_position: (i64, i64)) -> bool {
    let mut tortoise = starting_position;
    let mut hare = tortoise;

    let mut tortoise_direction: (i64, i64) = (0, -1);
    let mut hare_direction = tortoise_direction;

    let check_step_and_update = |p: (i64, i64), d: (i64, i64)| {
        let mut direction = d;
        loop {
            let new = (p.0 + direction.0, p.1 + direction.1);
            if let Some(x) = grid.at(new.0, new.1) {
                if x == '#' {
                    direction = next_direction(direction);
                } else {
                    return Some((new, direction));
                }
            } else {
                return None;
            }
        }
    };

    loop {
        let res = check_step_and_update(hare, hare_direction);
        if res.is_none() {
            return false;
        }

        hare = res.unwrap().0;
        hare_direction = res.unwrap().1;

        if hare == tortoise && hare_direction == tortoise_direction {
            return true;
        }

        let res = check_step_and_update(hare, hare_direction);
        if res.is_none() {
            return false;
        }

        hare = res.unwrap().0;
        hare_direction = res.unwrap().1;

        let res = check_step_and_update(tortoise, tortoise_direction);
        if res.is_none() {
            return false;
        }

        tortoise = res.unwrap().0;
        tortoise_direction = res.unwrap().1;

        if hare == tortoise && hare_direction == tortoise_direction {
            return true;
        }
    }
}

fn part2(grid: &Grid) -> i64 {
    let mut start = grid.find_starting_position();
    let mut grid_copy = grid.clone();

    let start_position = start;

    let mut direction: (i64, i64) = (0, -1);

    let mut loop_points = HashSet::new();
    loop {
        let new_position = (start.0 + direction.0, start.1 + direction.1);
        if let Some(x) = grid.at(new_position.0, new_position.1) {
            if x == '#' {
                // turning_points.insert((start, direction));
                direction = next_direction(direction);
                continue;
            } else {
                if new_position != start_position {
                    *grid_copy.at_mut(new_position.0, new_position.1) = '#';
                    if detect_loop(&grid_copy, start_position) {
                        loop_points.insert(new_position);
                    }
                    *grid_copy.at_mut(new_position.0, new_position.1) = '.';
                }
                start = new_position;
            }
        } else {
            break;
        }
    }

    loop_points.len() as i64
}

fn part1(grid: &Grid) -> i64 {
    let mut start = grid.find_starting_position();
    let mut direction: (i64, i64) = (0, -1);

    let mut visited = HashSet::new();
    visited.insert(start);

    loop {
        let new_position = (start.0 + direction.0, start.1 + direction.1);

        if let Some(x) = grid.at(new_position.0, new_position.1) {
            if x == '#' {
                direction = next_direction(direction);
                continue;
            } else {
                start = new_position;
                visited.insert(start);
            }
        } else {
            break;
        }
    }

    visited.len() as i64
}

fn main() {
    let input = include_str!("../input.txt");
    let grid = Grid::new(input);

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}
