
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Op
{
    XOR,
    AND,
    OR
}

#[derive(Debug, Clone)]
struct Node
{
    lhs: Option<String>,
    rhs: Option<String>,
    value: Option<i64>,
    op: Option<Op>
}

fn get_node_value(current: &String, wires: &HashMap<String, Node>) -> i64
{
    let node = wires.get(current).unwrap();

    if node.op.is_none() {
        return node.value.unwrap();
    }

    let lhs_value = get_node_value(node.lhs.as_ref().unwrap(), wires);
    let rhs_value = get_node_value(node.rhs.as_ref().unwrap(), wires);

    match node.op.as_ref().unwrap() {
        Op::AND => lhs_value & rhs_value,
        Op::XOR => lhs_value ^ rhs_value,
        Op::OR => lhs_value | rhs_value
    }
}

fn part1(wires: &HashMap<String, Node>) -> i64
{
    let mut accumulator = 0;

    for i in (0..64).rev() {
        let key = "z".to_string() + &format!("{:02}", i);

        if wires.contains_key(&key) {
            let value = get_node_value(&key, wires);

            accumulator += value * 2_i64.pow(i)
        }
    }
    accumulator
}

fn part2(mut wires: HashMap<String, Node>) -> i64
{
    let start_wires = wires.clone();

    let mut accumulator = 0;

    for i in 0..64 {
        let y_key = "y".to_string() + &format!("{:02}", i);
        let x_key = "x".to_string() + &format!("{:02}", i);

        if !wires.contains_key(&x_key) || !wires.contains_key(&y_key)
        {
            continue;
        }

        wires.get_mut(&x_key).as_mut().unwrap().value = Some(0);
        wires.get_mut(&y_key).as_mut().unwrap().value = Some(0);
    }

    for i in 0..64 {
        let z_key = "z".to_string() + &format!("{:02}", i);
        let y_key = "y".to_string() + &format!("{:02}", i);
        let x_key = "x".to_string() + &format!("{:02}", i);

        if !wires.contains_key(&z_key) || !wires.contains_key(&x_key) || !wires.contains_key(&y_key)
        {
            continue;
        }

        // Checking truth table

        let old_x = wires.get(&x_key).unwrap().value.unwrap();
        let old_y = wires.get(&y_key).unwrap().value.unwrap();


        for j in 0..4 {
            let new_x = j % 2;
            let new_y = j / 2;

            if i == 0 {
                wires.get_mut(&x_key).as_mut().unwrap().value = Some(new_x);
                wires.get_mut(&y_key).as_mut().unwrap().value = Some(new_y);

                let value = get_node_value(&z_key, &wires);
                let correct_value = new_x ^ new_y;
                println!("Bit:{}, {}, TRUTH TABLE: X: {} Y: {}, Z: {} vs: {}", i, if value == correct_value { "CORRECT" } else { "WRONG" }, j % 2, j / 2, value, correct_value);
            }
            else {
                let y_prev_key = "y".to_string() + &format!("{:02}", i - 1);
                let x_prev_key = "x".to_string() + &format!("{:02}", i - 1);
                for c in 0..4 {
                    let prev_x = c % 2;
                    let prev_y = c / 2;

                    wires.get_mut(&x_prev_key).as_mut().unwrap().value = Some(prev_x);
                    wires.get_mut(&y_prev_key).as_mut().unwrap().value = Some(prev_y);

    
                    let carry_in = (prev_x & prev_y) | (prev_x ^ prev_y);
                    let value = get_node_value(&z_key, &wires);
                    let correct_value = new_x ^ new_y ^ carry_in;
                    println!("Bit:{}, TRUTH TABLE: {}, CARRY: {}, X: {} Y: {}, Z: {} vs: {}", i, if value == correct_value { "CORRECT" } else { "WRONG" }, carry_in, j % 2, j / 2, value, correct_value);
                }

                wires.get_mut(&x_prev_key).as_mut().unwrap().value = Some(0);
                wires.get_mut(&y_prev_key).as_mut().unwrap().value = Some(0);
            }
            
        }
        println!();

        wires.get_mut(&x_key).as_mut().unwrap().value = Some(0);
        wires.get_mut(&y_key).as_mut().unwrap().value = Some(0);
        
        // println!("{}, {}", key, value);
        // accumulator += value * 2_i64.pow(i)
    }
    accumulator
}

fn main() {
    let input = include_str!("../input.txt");

    let mut wires = HashMap::new();

    input.split("\n\n").next().unwrap().lines().for_each(|line| {
        let mut it = line.split(": ");

        let wire_name = it.next().unwrap().to_string();
        let value = it.next().unwrap().parse::<i64>().unwrap();

        wires.insert(wire_name, Node { lhs: None, rhs: None, value: Some(value), op: None });
    });

    input.split("\n\n").nth(1).unwrap().lines().for_each(|line| {
        let mut it = line.split(" -> ");

        let parents = it.next().unwrap().split(" ").map(|x| x.to_string()).collect::<Vec<_>>();
        let wire_name = it.next().unwrap().to_string();
        
        let op = match parents[1].as_str() {
            "XOR" => Op::XOR,
            "AND" => Op::AND,
            "OR" => Op::OR,
            _ => panic!()
        };

        wires.insert(wire_name, Node { lhs: Some(parents[0].clone()), rhs: Some(parents[2].clone()), value: None, op: Some(op) });
    });

    // println!("{}", part1(&wires));
    println!("{}", part2(wires));
}
