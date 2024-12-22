use std::collections::{HashSet, VecDeque};

#[derive(Clone)]
struct Grid<T> {
    grid: Vec<T>,
    width: i64,
    height: i64,
}

impl<T> Grid<T>
where
    T: Copy,
{
    fn new<X: Fn((usize, char)) -> T>(input: &str, map_fn: X) -> Grid<T> {
        let height = input.lines().count() as i64;
        let width = input.lines().next().unwrap().len() as i64;

        Grid::<T> {
            grid: input
                .chars()
                .filter(|x| *x != '\n')
                .enumerate()
                .map(map_fn)
                .collect(),
            width,
            height,
        }
    }

    fn out_of_bounds(&self, a: (i64, i64)) -> bool {
        if a.0 < 0 || a.0 >= self.width {
            return true;
        }
        if a.1 < 0 || a.1 >= self.height {
            return true;
        }

        false
    }

    fn at(&self, a: (i64, i64)) -> Option<T> {
        if self.out_of_bounds(a) {
            None
        } else {
            Some(self.grid[(a.0 as usize) + (self.width * a.1) as usize])
        }
    }

    fn at_unchecked(&self, a: (i64, i64)) -> T {
        self.grid[(a.0 as usize) + (self.width * a.1) as usize]
    }

    fn at_mut(&mut self, a: (i64, i64)) -> &mut T {
        &mut self.grid[(a.0 as usize) + (self.width * a.1) as usize]
    }

    fn neighbours() -> [(i64, i64); 4] {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
    }
}

fn add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

fn find_one_tile_thick_walls(grid: &Grid<char>) -> Vec<(i64, i64)> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.at_unchecked((x, y)) == 'S' {
                start = (x, y);
            }
            if grid.at_unchecked((x, y)) == 'E' {
                end = (x, y);
            }
        }
    }

    let mut visited = Grid::<i64> {
        grid: vec![i64::MAX; (grid.width * grid.height) as usize],
        width: grid.width,
        height: grid.height,
    };
    *visited.at_mut(start) = 0;

    let mut queue = VecDeque::new();

    queue.push_back(vec![start; 1]);

    let mut walls = Vec::new();

    while let Some(path) = queue.pop_front() {
        let node = *path.last().unwrap();

        if node == end {
            break;
        }

        let cost = visited.at_unchecked(node);

        for neighbour in Grid::<char>::neighbours() {
            let next = add(node, neighbour);

            if grid.at(next).unwrap_or('#') == '#' {
                let next_again = add(next, neighbour);
                if grid.at(next_again).unwrap_or('#') != '#' {
                    walls.push(next);
                }
                continue;
            }

            let current_cost = visited.at(next).unwrap_or(i64::MAX);

            if cost + 1 < current_cost {
                *visited.at_mut(next) = cost + 1;
                let mut cloned = path.clone();
                cloned.push(next);
                queue.push_back(cloned);
            }
        }
    }

    walls
}

fn bfs_pathfind(start: (i64, i64), end: (i64, i64), grid: &Grid<char>) -> Grid<i64> {
    let mut visited = Grid::<i64> {
        grid: vec![i64::MAX; (grid.width * grid.height) as usize],
        width: grid.width,
        height: grid.height,
    };
    
    *visited.at_mut(start) = 0;

    let mut queue = VecDeque::new();

    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if node == end {
            break;
        }

        let cost = visited.at_unchecked(node);

        for neighbour in Grid::<char>::neighbours() {
            let next = add(node, neighbour);

            if grid.at(next).unwrap_or('#') == '#' {
                continue;
            }

            let current_cost = visited.at(next).unwrap_or(i64::MAX);

            if cost + 1 < current_cost {
                *visited.at_mut(next) = cost + 1;
                queue.push_back(next);
            }
        }
    }

    visited
}

fn part1(start_grid: &Grid<char>) -> i64 {
    let mut grid = start_grid.clone();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.at_unchecked((x, y)) == 'S' {
                start = (x, y);
            }
            if grid.at_unchecked((x, y)) == 'E' {
                end = (x, y);
            }
        }
    }

    let start_length = bfs_pathfind(start, end, &grid).at_unchecked(end);

    let walls = HashSet::<(i64, i64)>::from_iter(find_one_tile_thick_walls(&grid));

    walls
        .iter()
        .filter(|wall| {
            *grid.at_mut(**wall) = '.';
            let new_length = bfs_pathfind(start, end, &grid).at_unchecked(end);

            *grid.at_mut(**wall) = '#';
            start_length - new_length >= 100
        })
        .count() as i64
}

fn part2(grid: &Grid<char>) -> i64 {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.at_unchecked((x, y)) == 'S' {
                start = (x, y);
            }
            if grid.at_unchecked((x, y)) == 'E' {
                end = (x, y);
            }
        }
    }

    let visited = bfs_pathfind(start, end, grid);

    let mut cheats = Vec::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            for dy in -20..=20 {
                for dx in -20..=20 {
                    let cheat_end_value = visited.at((x + dx, y + dy)).unwrap_or(i64::MAX);
                    if cheat_end_value != i64::MAX && cheat_end_value > visited.at_unchecked((x, y))
                    {
                        cheats.push(((x, y), (x + dx, y + dy)));
                    }
                }
            }
        }
    }

    cheats
        .iter()
        .filter(|(start, end)| {
            let cheat_length = (end.0 - start.0).abs() + (end.1 - start.1).abs();

            if cheat_length > 20 {
                return false;
            }

            let start_time = visited.at_unchecked(*start);
            let end_time = visited.at_unchecked(*end);

            (end_time - start_time - cheat_length) >= 100
        })
        .collect::<HashSet<_>>()
        .len() as i64
    // visited.at_unchecked(end)
}

fn main() {
    let grid = Grid::new(include_str!("../input.txt"), |(_i, x)| x);

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}
