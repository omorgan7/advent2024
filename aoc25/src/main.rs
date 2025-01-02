use std::collections::HashSet;

struct Grid<T> {
    grid: Vec<T>,
    width: i64,
    height: i64,
}

impl<T> Grid<T>
where
    T: Copy + std::fmt::Display,
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

    fn render(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.at_unchecked((x, y)));
            }
            println!()
        }
        println!();
    }
}

fn main() {
    let grids = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| Grid::<char>::new(s, |x| x))
        .collect::<Vec<_>>();

    let locks = grids.iter().filter(|g| {
        g.at_unchecked((0, 0)) == '#'
    }).collect::<Vec<_>>();

    let keys = grids.iter().filter(|g| {
        g.at_unchecked((g.width - 1, g.height - 1)) == '#'
    }).collect::<Vec<_>>();

    let count_columns = |x: &Grid<char>| -> [i64; 5] {

        let mut column_count = [-1,-1,-1,-1,-1];

        for column in 0..5_i64 {
            for y in 0..x.height {
                column_count[column as usize] += if x.at_unchecked((column, y)) == '#' {
                    1
                } else {
                    0
                };
            }
        }

        column_count
        
    };

    let mut count = 0;

    for l in &locks {
        let lock_count = count_columns(l);
        for k in &keys {
            let key_count = count_columns(k);
            if key_count.iter().zip(lock_count).all(|(x, y)| {
                x + y <= 5
            }) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
