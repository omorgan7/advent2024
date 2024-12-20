use std::collections::{HashMap, HashSet, VecDeque};

struct Grid<T> {
    grid: Vec<T>,
    width: i64,
    height: i64,
}

impl<T> Grid<T>
where
    T: Copy,
{
    fn new<X: Fn(char) -> T>(input: &str, map_fn: X) -> Grid<T> {
        let height = input.lines().count() as i64;
        let width = input.lines().next().unwrap().len() as i64;

        Grid::<T> {
            grid: input.chars().filter(|x| *x != '\n').map(map_fn).collect(),
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

    fn directions(a: (i64, i64)) -> [(i64, (i64, i64)); 3] {
        if a.0 == 0 {
            [(1, a), (1001, (1, 0)), (1001, (-1, 0))]
        } else {
            [(1, a), (1001, (0, 1)), (1001, (0, -1))]
        }
    }
}

fn add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

fn bfs_pathfind(grid: &Grid<char>) -> Vec<Vec<((i64, i64), (i64, i64), i64)>> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.at_unchecked((x, y)) == 'E' {
                end = (x, y);
            }

            if grid.at_unchecked((x, y)) == 'S' {
                start = (x, y);
            }
        }
    }

    let mut visited: HashMap<(i64, i64), Grid<i64>> = HashMap::new();

    visited.insert(
        (1, 0),
        Grid::<i64> {
            grid: vec![i64::MAX; (grid.width * grid.height) as usize],
            width: grid.width,
            height: grid.height,
        },
    );
    visited.insert(
        (-1, 0),
        Grid::<i64> {
            grid: vec![i64::MAX; (grid.width * grid.height) as usize],
            width: grid.width,
            height: grid.height,
        },
    );
    visited.insert(
        (0, 1),
        Grid::<i64> {
            grid: vec![i64::MAX; (grid.width * grid.height) as usize],
            width: grid.width,
            height: grid.height,
        },
    );
    visited.insert(
        (0, -1),
        Grid::<i64> {
            grid: vec![i64::MAX; (grid.width * grid.height) as usize],
            width: grid.width,
            height: grid.height,
        },
    );

    for (_, v) in visited.iter_mut() {
        *v.at_mut(start) = 0;
    }

    let mut queue = VecDeque::new();

    queue.push_back(vec![(start, (1, 0), 0); 1]);

    let mut final_paths: Vec<Vec<((i64, i64), (i64, i64), i64)>> = Vec::new();

    while let Some(path) = queue.pop_front() {
        let (node, direction, cost) = *path.last().unwrap();

        if node == end {
            let mut should_insert = true;

            for other_path in final_paths.iter() {
                if cost > other_path.last().unwrap().2 {
                    should_insert = false;
                    break;
                }
            }

            if !should_insert {
                continue;
            }

            final_paths.retain(|other_path| other_path.last().unwrap().2 <= cost);

            final_paths.push(path);
            continue;
        }

        // let cost = visited.get(&direction).unwrap().at_unchecked(node);

        for neighbour in Grid::<char>::directions(direction) {
            let next = add(node, neighbour.1);

            if grid.at(next).unwrap_or('#') == '#' {
                continue;
            }

            let current_cost = visited
                .get(&neighbour.1)
                .unwrap()
                .at(next)
                .unwrap_or(i64::MAX);
            let new_cost = cost + neighbour.0;
            if new_cost <= current_cost {
                *visited.get_mut(&neighbour.1).unwrap().at_mut(next) = new_cost;
                let mut cloned = path.clone();
                cloned.push((next, neighbour.1, new_cost));
                queue.push_back(cloned);
            }
        }
    }

    final_paths
}

fn part1(grid: &Grid<char>) -> i64 {
    bfs_pathfind(grid).last().unwrap().last().unwrap().2
}

fn part2(grid: &Grid<char>) -> i64 {
    let paths = bfs_pathfind(grid);
    paths
        .iter()
        .flatten()
        .map(|path| path.0)
        .collect::<HashSet<(i64, i64)>>()
        .len() as i64
}

fn main() {
    let grid = Grid::new(include_str!("../input.txt"), |x| x);
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}
