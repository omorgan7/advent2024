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

fn bfs_pathfind(grid: &Grid<char>) -> Grid<i64> {
    let start = (0, 0);

    let mut visited = Grid::<i64> {
        grid: vec![i64::MAX; (grid.width * grid.height) as usize],
        width: grid.width,
        height: grid.height,
    };
    *visited.at_mut(start) = 0;

    let mut queue = VecDeque::new();

    queue.push_back(start);

    let end = (grid.width - 1, grid.height - 1);

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

fn part1(input: &str) -> i64 {
    let corrupted_bytes = input
        .lines()
        .take(1024)
        .map(|line| {
            let mut it = line.split(",").map(|x| x.parse::<i64>().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect::<HashSet<(i64, i64)>>();

    let width = 71_i64;
    let height = 71_i64;

    let mut g = vec!['.'; (width * height) as usize];

    for i in 0..g.len() {
        if corrupted_bytes.contains(&(i as i64 % width, i as i64 / height)) {
            g[i] = '#';
        }
    }

    let grid = Grid::<char> {
        grid: g,
        width,
        height,
    };
    let visited = bfs_pathfind(&grid);

    visited.at_unchecked((visited.width - 1, visited.height - 1))
}

fn part2(input: &str) -> (i64, i64) {
    let corrupted_bytes = input
        .lines()
        .map(|line| {
            let mut it = line.split(",").map(|x| x.parse::<i64>().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();

    let width = 71_i64;
    let height = 71_i64;

    let mut g = vec!['.'; (width * height) as usize];

    for b in &corrupted_bytes {
        g[(b.0 + width * b.1) as usize] = '#';
    }

    let mut grid = Grid::<char> {
        grid: g,
        width,
        height,
    };

    for i in (0..corrupted_bytes.len()).rev() {
        *grid.at_mut(corrupted_bytes[i]) = '.';

        let visited = bfs_pathfind(&grid);

        if visited.at_unchecked((visited.width - 1, visited.height - 1)) != i64::MAX {
            return corrupted_bytes[i];
        }
    }

    panic!()
}

fn main() {
    println!("{}", part1(include_str!("../input.txt")));
    println!("{:?}", part2(include_str!("../input.txt")));
}
