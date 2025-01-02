#[derive(Clone, PartialEq, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    A,
}

fn print_direction(directions: &[Direction])
{
    for d in directions {
        print!("{}", match d
        {
            Direction::A => 'A', 
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Up => '^'
        });
    }
    println!();
}

#[test]
fn test1() {
    assert_eq!(numeric_directions('3', '9'), vec![Direction::Up, Direction::Up]);
    assert_eq!(numeric_directions('2', '9'), vec![Direction::Right, Direction::Up, Direction::Up]);
    assert_eq!(numeric_directions('A', '4'), vec![Direction::Up, Direction::Up, Direction::Left, Direction::Left]);
    assert_eq!(numeric_directions('A', '7'), vec![Direction::Up, Direction::Up, Direction::Up, Direction::Left, Direction::Left]);
    assert_eq!(numeric_directions('7', '0'), vec![Direction::Right, Direction::Down, Direction::Down, Direction::Down]);
    assert_eq!(numeric_directions('1', '0'), vec![Direction::Right, Direction::Down]);
    assert_eq!(numeric_directions('0', '1'), vec![Direction::Up, Direction::Left]);
    assert_eq!(numeric_directions('6', '6'), vec![]);
    assert_eq!(numeric_directions('4', 'A'), vec![Direction::Right, Direction::Right, Direction::Down, Direction::Down]);
    assert_eq!(numeric_directions('4', '5'), vec![Direction::Right]);
    assert_eq!(numeric_directions('5', '6'), vec![Direction::Right]);
}

// Numeric
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
fn numeric_directions(from: char, to: char) -> Vec<Direction> {
    let get_coordinates = |c: char| -> (i64, i64) {
        if c == '0' {
            (1, 3)
        } else if c == 'A' {
            (2, 3)
        } else {
            let num = c.to_digit(10).unwrap() as i64;
            ((2 + num % 3) % 3, 2 - (num - 1) / 3)
        }
    };

    let to_coordinates = get_coordinates(to);
    let from_coordinates = get_coordinates(from);

    let mut difference = (
        to_coordinates.0 - from_coordinates.0,
        to_coordinates.1 - from_coordinates.1,
    );

    let mut directions = Vec::new();
    
    while difference.0 > 0 {
        directions.push(Direction::Right);
        difference.0 -= 1;
    }

    while difference.1 > 0 {
        directions.push(Direction::Down);
        difference.1 -= 1;
    }

    while difference.1 < 0 {
        directions.push(Direction::Up);
        difference.1 += 1;
    }
    
    while difference.0 < 0 {
        directions.push(Direction::Left);
        difference.0 += 1;
    }

    directions
}

#[test]
fn test2() {
    assert_eq!(directional_directions(Direction::Up, Direction::Right), vec![Direction::Down, Direction::Right]);
    assert_eq!(directional_directions(Direction::Up, Direction::A), vec![Direction::Right]);
    assert_eq!(directional_directions(Direction::Up, Direction::Down), vec![Direction::Down]);
    assert_eq!(directional_directions(Direction::Up, Direction::Left), vec![Direction::Down, Direction::Left]);
    assert_eq!(directional_directions(Direction::Up, Direction::Down), vec![Direction::Down]);
    assert_eq!(directional_directions(Direction::Right, Direction::Left), vec![Direction::Left, Direction::Left]);
    assert_eq!(directional_directions(Direction::Left, Direction::A), vec![Direction::Right, Direction::Right, Direction::Up]);
    assert_eq!(directional_directions(Direction::A, Direction::Left), vec![Direction::Down, Direction::Left, Direction::Left]);
    assert_eq!(directional_directions(Direction::Left, Direction::Up), vec![Direction::Right, Direction::Up])
}

// Directional
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
fn directional_directions(from: Direction, to: Direction) -> Vec<Direction>
{
    let get_coordinates = |c: Direction| -> (i64, i64) {
        match c {
            Direction::Up => (1, 0),
            Direction::A => (2, 0),
            Direction::Left => (0, 1),
            Direction::Down => (1, 1),
            Direction::Right => (2, 1)
        }
    };

    let to_coordinates = get_coordinates(to);
    let from_coordinates = get_coordinates(from);

    let mut difference = (
        to_coordinates.0 - from_coordinates.0,
        to_coordinates.1 - from_coordinates.1,
    );

    let mut directions = Vec::new();

    while difference.1 > 0 {
        directions.push(Direction::Down);
        difference.1 -= 1;
    }
    
    while difference.0 > 0 {
        directions.push(Direction::Right);
        difference.0 -= 1;
    }

    while difference.1 < 0 {
        directions.push(Direction::Up);
        difference.1 += 1;
    }

    while difference.0 < 0 {
        directions.push(Direction::Left);
        difference.0 += 1;
    }

    directions
}

fn parse_digit(code: &[char]) -> i64 {
    let s= code[0..code.len() - 1].iter().collect::<String>();
    s.parse::<i64>().unwrap()
}

fn part1(codes: &[Vec<char>]) -> i64
{
    codes.iter().map(|code| {
        let mut robot1 = Vec::new();

        let mut current_digit = 'A';

        for i in 0..code.len() {
            let next = code[i];
            robot1.append(&mut numeric_directions(current_digit, next));
            robot1.push(Direction::A);
            current_digit = next;

        }

        let mut robot2 = Vec::new();
        let mut current_direction = Direction::A;

        for i in 0..robot1.len() {
            let next = robot1[i];
            robot2.append(&mut directional_directions(current_direction, next));
            robot2.push(Direction::A);
            current_direction = next;
        }

        let mut robot3 = Vec::new();
        let mut current_direction = Direction::A;

        for i in 0..robot2.len() {
            let next = robot2[i];
            let mut tmp = directional_directions(current_direction, next);
            tmp.push(Direction::A);
            print_direction(&tmp);
            robot3.append(&mut tmp);
            // robot3.push(Direction::A);
            current_direction = next;
        }

        // print_direction(&robot1);
        // print_direction(&robot2);
        // print_direction(&robot3);
        // print_direction(&you);
        println!("{} * {}", robot3.len(), parse_digit(code));
        parse_digit(code) *  robot3.len() as i64

    }).sum::<i64>()
}

fn main() {
    let input = include_str!("../input.txt");
    let codes = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();

    println!("{}", part1(&codes));
}
