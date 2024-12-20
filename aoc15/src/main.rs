use std::collections::HashSet;
use std::ops;
use std::vec::Vec;

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

impl ops::Sub<Vector2> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum State {
    Occupied,
    Box,
    Wall,
    Empty,
    BoxLHS,
    BoxRHS,
}

struct Grid {
    grid: Vec<State>,
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
    fn to_bigger_grid(&self) -> Grid {
        let mut new_grid = Vec::<State>::new();

        for g in &self.grid {
            match g {
                State::Wall => {
                    new_grid.push(State::Wall);
                    new_grid.push(State::Wall);
                }
                State::Box => {
                    new_grid.push(State::BoxLHS);
                    new_grid.push(State::BoxRHS);
                }
                State::Occupied => {
                    new_grid.push(State::Occupied);
                    new_grid.push(State::Empty);
                }
                State::Empty => {
                    new_grid.push(State::Empty);
                    new_grid.push(State::Empty);
                }
                _ => panic!(),
            }
        }

        Grid {
            grid: new_grid,
            width: self.width * 2,
            height: self.height,
        }
    }

    fn new(input: &str) -> Grid {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        let grid: Vec<_> = input
            .chars()
            .filter_map(|x| match x {
                '#' => Some(State::Wall),
                'O' => Some(State::Box),
                '@' => Some(State::Occupied),
                '.' => Some(State::Empty),
                _ => None,
            })
            .collect();

        Grid {
            grid,
            width,
            height,
        }
    }

    fn find_starting_position(&self) -> Vector2 {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[x + self.width * y] == State::Occupied {
                    return Vector2 {
                        x: x as i64,
                        y: y as i64,
                    };
                }
            }
        }
        panic!()
    }

    fn at_mut(&mut self, a: Vector2) -> &mut State {
        &mut self.grid[(a.x as usize) + self.width * (a.y as usize)]
    }

    fn at_unchecked(&self, a: Vector2) -> State {
        self.grid[(a.x as usize) + self.width * (a.y as usize)]
    }
}

fn part1(mut grid: Grid, moves: &[Vector2]) -> i64 {
    let mut position = grid.find_starting_position();
    for m in moves {
        let next_position = position + *m;
        match grid.at_unchecked(next_position) {
            State::Empty => {
                *grid.at_mut(position) = State::Empty;
                *grid.at_mut(next_position) = State::Occupied;
                position = next_position;
            }
            State::Wall => {}
            State::Box => {
                let mut next_box = next_position;
                let mut box_state = grid.at_unchecked(next_box);

                while box_state == State::Box {
                    next_box = next_box + *m;
                    box_state = grid.at_unchecked(next_box);
                }

                if box_state == State::Empty {
                    *grid.at_mut(next_box) = State::Box;
                    *grid.at_mut(next_position) = State::Occupied;
                    *grid.at_mut(position) = State::Empty;
                    position = next_position;
                }
            }
            _ => panic!(),
        }
    }

    let mut gps = 0;
    for y in 0..grid.height as i64 {
        for x in 0..grid.width as i64 {
            if grid.at_unchecked(Vector2 { x, y }) == State::Box {
                gps += y * 100 + x;
            }
        }
    }

    gps
}

fn check_movable(position: Vector2, parent: Vector2, direction: Vector2, grid: &Grid) -> bool {
    let box_state = grid.at_unchecked(position);

    if direction.y == 0 {
        match box_state {
            State::BoxLHS | State::BoxRHS => {
                check_movable(position + direction, parent, direction, grid)
            }
            State::Wall => false,
            State::Empty => true,
            _ => panic!(),
        }
    } else {
        match box_state {
            State::BoxLHS => {
                let lhs_check = check_movable(position + direction, parent, direction, grid);
                if parent != position + (Vector2 { x: 1, y: 0 }) {
                    lhs_check
                        && check_movable(position + Vector2 { x: 1, y: 0 }, parent, direction, grid)
                } else {
                    lhs_check
                }
            }
            State::BoxRHS => {
                let rhs_check = check_movable(position + direction, parent, direction, grid);

                if parent != position + (Vector2 { x: -1, y: 0 }) {
                    check_movable(
                        position + Vector2 { x: -1, y: 0 },
                        position,
                        direction,
                        grid,
                    ) && rhs_check
                } else {
                    rhs_check
                }
            }
            State::Wall => false,
            State::Empty => true,
            _ => panic!(),
        }
    }
}

fn change_state_recursive(
    position: Vector2,
    direction: Vector2,
    grid: &mut Grid,
    original_grid: &Grid,
    visited: &mut HashSet<Vector2>,
) {
    let box_state = original_grid.at_unchecked(position);
    visited.insert(position);

    if direction.y == 0 {
        match box_state {
            State::BoxLHS | State::BoxRHS => {
                change_state_recursive(
                    position + direction,
                    direction,
                    grid,
                    original_grid,
                    visited,
                );
                *grid.at_mut(position + direction) = box_state;
            }
            State::Empty => {}
            _ => panic!(),
        }
    } else {
        match box_state {
            State::BoxLHS => {
                change_state_recursive(
                    position + direction,
                    direction,
                    grid,
                    original_grid,
                    visited,
                );
                if !visited.contains(&(position + (Vector2 { x: 1, y: 0 }))) {
                    change_state_recursive(
                        position + Vector2 { x: 1, y: 0 },
                        direction,
                        grid,
                        original_grid,
                        visited,
                    );
                    *grid.at_mut(position + Vector2 { x: 1, y: 0 }) = State::Empty;
                }
                *grid.at_mut(position + direction) = box_state;
            }
            State::BoxRHS => {
                change_state_recursive(
                    position + direction,
                    direction,
                    grid,
                    original_grid,
                    visited,
                );
                if !visited.contains(&(position + (Vector2 { x: -1, y: 0 }))) {
                    change_state_recursive(
                        position + Vector2 { x: -1, y: 0 },
                        direction,
                        grid,
                        original_grid,
                        visited,
                    );
                    *grid.at_mut(position + Vector2 { x: -1, y: 0 }) = State::Empty;
                }
                *grid.at_mut(position + direction) = box_state;
            }
            State::Empty => {}
            _ => panic!(),
        }
    }
}

fn part2(mut grid: Grid, moves: &[Vector2]) -> i64 {
    let mut position = grid.find_starting_position();
    for m in moves {
        let next_position = position + *m;
        let state = grid.at_unchecked(next_position);
        match state {
            State::Empty => {
                *grid.at_mut(position) = State::Empty;
                *grid.at_mut(next_position) = State::Occupied;
                position = next_position;
            }
            State::Wall => {}
            State::BoxLHS | State::BoxRHS => {
                if check_movable(next_position, position, *m, &grid) {
                    let cloned = grid.clone();
                    let mut visited = HashSet::<Vector2>::new();
                    change_state_recursive(next_position, *m, &mut grid, &cloned, &mut visited);

                    *grid.at_mut(next_position) = State::Occupied;
                    *grid.at_mut(position) = State::Empty;
                    position = next_position;
                }
            }
            _ => panic!(),
        }
    }

    let mut gps = 0;
    for y in 0..grid.height as i64 {
        for x in 0..grid.width as i64 {
            if grid.at_unchecked(Vector2 { x, y }) == State::BoxLHS {
                gps += y * 100 + x;
            }
        }
    }

    gps
}

fn main() {
    let input = include_str!("../input.txt");

    let grid = Grid::new(input.split("\n\n").next().unwrap());

    let moves = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .chars()
        .filter_map(|x| match x {
            '^' => Some(Vector2 { x: 0, y: -1 }),
            'v' => Some(Vector2 { x: 0, y: 1 }),
            '>' => Some(Vector2 { x: 1, y: 0 }),
            '<' => Some(Vector2 { x: -1, y: 0 }),
            _ => None,
        })
        .collect::<Vec<Vector2>>();

    println!("{}", part1(grid.clone(), &moves));
    println!("{}", part2(grid.to_bigger_grid(), &moves));
}
