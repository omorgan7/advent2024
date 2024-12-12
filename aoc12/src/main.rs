use multimap::MultiMap;
use std::collections::HashSet;
use std::collections::VecDeque;
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

    fn at(&self, x: i64, y: i64) -> Option<char> {
        if self.out_of_bounds(x, y) {
            None
        } else {
            Some(self.grid[(x as usize) + self.width * (y as usize)])
        }
    }

    fn at_unchecked(&self, x: i64, y: i64) -> char {
        self.grid[(x as usize) + self.width * (y as usize)]
    }

    fn neighbours() -> [(i64, i64); 4] {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
    }
}

fn add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

fn orthogonal_direction(d: (i64, i64)) -> (i64, i64) {
    if d.0 == 0 {
        (1, 0)
    } else {
        (0, 1)
    }
}
fn flip(x: (i64, i64)) -> (i64, i64) {
    (-x.0, -x.1)
}

fn part1(grid: &Grid) -> i64 {
    let mut patches = MultiMap::<char, HashSet<(i64, i64)>>::new();

    let mut queue = VecDeque::new();

    for y in 0..grid.height as i64 {
        for x in 0..grid.width as i64 {
            let p = grid.at_unchecked(x, y);

            let mut need_processing = true;

            if patches.contains_key(&p) {
                let p_patches = patches.get_vec(&p).unwrap();
                for pp in p_patches {
                    if pp.contains(&(x, y)) {
                        need_processing = false;
                        break;
                    }
                }
            }

            if !need_processing {
                continue;
            }

            queue.clear();

            queue.push_back((x, y));
            let mut patch_squares = HashSet::<(i64, i64)>::new();
            while let Some(next) = queue.pop_back() {
                patch_squares.insert(next);

                for n in Grid::neighbours() {
                    let neighbour = add(next, n);

                    if !patch_squares.contains(&neighbour)
                        && grid.at(neighbour.0, neighbour.1).unwrap_or('#') == p
                    {
                        queue.push_back(neighbour);
                    }
                }
            }
            patches.insert(p, patch_squares);
        }
    }

    patches
        .iter_all()
        .map(|(key, values)| {
            values
                .iter()
                .map(|patch| {
                    let area = patch.len() as i64;

                    area * if patch.len() == 1 {
                        4
                    } else {
                        patch
                            .iter()
                            .map(|p| {
                                let mut neighbour_count = 0;
                                for n in Grid::neighbours() {
                                    let neighbour = add(*p, n);
                                    if *key != grid.at(neighbour.0, neighbour.1).unwrap_or('#') {
                                        neighbour_count += 1;
                                    }
                                }
                                neighbour_count
                            })
                            .sum::<i64>()
                    }
                })
                .sum::<i64>()
        })
        .sum::<i64>()
}

fn part2(grid: &Grid) -> i64 {
    let mut patches = MultiMap::<char, HashSet<(i64, i64)>>::new();

    let mut queue = VecDeque::new();
    let mut stack = Vec::new();

    for y in 0..grid.height as i64 {
        for x in 0..grid.width as i64 {
            let p = grid.at_unchecked(x, y);

            let mut need_processing = true;

            if patches.contains_key(&p) {
                let p_patches = patches.get_vec(&p).unwrap();
                for pp in p_patches {
                    if pp.contains(&(x, y)) {
                        need_processing = false;
                        break;
                    }
                }
            }

            if !need_processing {
                continue;
            }

            queue.clear();

            queue.push_back((x, y));
            let mut patch_squares = HashSet::<(i64, i64)>::new();
            while let Some(next) = queue.pop_back() {
                patch_squares.insert(next);

                for n in Grid::neighbours() {
                    let neighbour = add(next, n);

                    if !patch_squares.contains(&neighbour)
                        && grid.at(neighbour.0, neighbour.1).unwrap_or('#') == p
                    {
                        queue.push_back(neighbour);
                    }
                }
            }
            patches.insert(p, patch_squares);
        }
    }

    let mut perimeters = HashSet::<((i64, i64), (i64, i64))>::new();

    patches
        .iter_all()
        .map(|(key, values)| {
            values
                .iter()
                .map(|patch| {
                    let area = patch.len() as i64;

                    area * if patch.len() == 1 {
                        4
                    } else {
                        patch
                            .iter()
                            .map(|p| {
                                let mut side_count = 0;

                                for n in Grid::neighbours() {
                                    let neighbour = add(*p, n);
                                    if *key != grid.at(neighbour.0, neighbour.1).unwrap_or('#') {
                                        stack.push((neighbour, orthogonal_direction(n), flip(n)));
                                        stack.push((
                                            neighbour,
                                            flip(orthogonal_direction(n)),
                                            flip(n),
                                        ));
                                    }

                                    let mut added_side = false;

                                    let mut this_perimeters = Vec::new();
                                    while let Some(perimeter_piece) = stack.pop() {
                                        if perimeters.contains(&(*p, perimeter_piece.0)) {
                                            continue;
                                        }

                                        let next = grid
                                            .at(perimeter_piece.0 .0, perimeter_piece.0 .1)
                                            .unwrap_or('#');
                                        let original_p = add(perimeter_piece.0, perimeter_piece.2);
                                        let original =
                                            grid.at(original_p.0, original_p.1).unwrap_or('#');

                                        if next != *key && original == *key {
                                            added_side = true;
                                            this_perimeters.push((original_p, perimeter_piece.0));
                                            stack.push((
                                                add(perimeter_piece.0, perimeter_piece.1),
                                                perimeter_piece.1,
                                                perimeter_piece.2,
                                            ));
                                        }
                                    }
                                    this_perimeters.iter().for_each(|p| {
                                        perimeters.insert(*p);
                                    });

                                    side_count += if added_side { 1 } else { 0 };
                                }

                                side_count
                            })
                            .sum::<i64>()
                    }
                })
                .sum::<i64>()
        })
        .sum::<i64>()
}

fn main() {
    let input = include_str!("../input.txt");

    let grid = Grid::new(input);

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}
