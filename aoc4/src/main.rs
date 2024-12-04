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

    fn at(&self, x: i64, y: i64) -> Option<char> {
        if x < 0 || x >= self.width as i64 {
            return None;
        }
        if y < 0 || y >= self.height as i64 {
            return None;
        }

        Some(self.grid[(x as usize) + self.width * (y as usize)])
    }

    fn neighbours() -> [(i64, i64); 8] {
        [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ]
    }
}

fn search_next(grid: &Grid, x: i64, y: i64, next: &[char], direction: (i64, i64)) -> bool {
    if next.is_empty() {
        return true;
    }

    if grid.at(x, y).unwrap_or('.') == next[0] {
        return search_next(
            grid,
            x + direction.0,
            y + direction.1,
            &next[1..],
            direction,
        );
    }

    false
}

fn part1(grid: &Grid) -> i64 {
    let search = ['X', 'M', 'A', 'S'];

    let mut count: i64 = 0;

    for y in 0..grid.height as i64 {
        for x in 0..grid.width as i64 {
            if grid.at(x, y).unwrap() != search[0] {
                continue;
            }

            for direction in Grid::neighbours() {
                if search_next(
                    grid,
                    x + direction.0,
                    y + direction.1,
                    &search[1..],
                    direction,
                ) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(grid: &Grid) -> i64 {
    let search = ['S', 'M'];

    let mut count: i64 = 0;

    let cardinal_pairs: [[(i64, i64); 4]; 4] = [
        [(-1, -1), (1, -1), (-1, 1), (1, 1)],
        [(-1, 1), (1, 1), (-1, -1), (1, -1)],
        [(-1, -1), (-1, 1), (1, -1), (1, 1)],
        [(1, -1), (1, 1), (-1, -1), (-1, 1)],
    ];

    for y in 0..grid.height as i64 {
        for x in 0..grid.width as i64 {
            if grid.at(x, y).unwrap() != 'A' {
                continue;
            }

            for pair in cardinal_pairs {
                let mut ok = false;
                if let Some(g0) = grid.at(x + pair[0].0, y + pair[0].1) {
                    if let Some(g1) = grid.at(x + pair[1].0, y + pair[1].1) {
                        if g0 != search[0] || g1 != search[0] {
                            continue;
                        } else {
                            ok = true;
                        }
                    }
                }

                if !ok {
                    continue;
                }
                ok = false;

                if let Some(g0) = grid.at(x + pair[2].0, y + pair[2].1) {
                    if let Some(g1) = grid.at(x + pair[3].0, y + pair[3].1) {
                        if g0 != search[1] || g1 != search[1] {
                            continue;
                        } else {
                            ok = true;
                        }
                    }
                }

                if ok {
                    count += 1;
                }

                break;
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("../input.txt");
    let grid = Grid::new(input);

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}
