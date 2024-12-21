#[repr(i64)]
#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug, Clone, Copy)]
enum Instruction
{
    adv = 0,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv,
    cdv
}

fn get_value(op: i64, a: i64, b: i64, c: i64) -> i64 
{
    if op < 4 {
        op
    } else {
        if op == 4{
            a
        } else if op == 5 {
            b
        } else if op == 6 {
            c
        } else {
            panic!();
        }
    }
}

fn part1(instructions: &[Instruction], operands: &[i64], mut a: i64, mut b: i64, mut c: i64) -> Vec<i64>
{
    let mut instruction_pointer: i64 = 0;

    let mut out = Vec::new();
    while (instruction_pointer as usize) < instructions.len() {
        let instruction = &instructions[instruction_pointer as usize];
        let operand = operands[instruction_pointer as usize];

        // println!("{:?}, {} {} {} {}", *instruction, a, b, c, operand);

        match *instruction {
            Instruction::adv => {
                let combo_op = get_value(operand, a, b, c);
                a = a / 2_i64.pow(combo_op as u32);
            }
            Instruction::bxl => {
                b = b ^ operand;
            }
            Instruction::bst => {
                let combo_op = get_value(operand, a, b, c);
                b = combo_op % 8;
            }
            Instruction::jnz => {
                if a != 0 {
                    instruction_pointer = operand / 2;
                }
            }
            Instruction::bxc => {
                b = b ^ c;
            }
            Instruction::out => {
                let combo_op = get_value(operand, a, b, c);
                out.push(combo_op % 8);
            }
            Instruction::bdv => {
                let combo_op = get_value(operand, a, b, c);
                b = a / 2_i64.pow(combo_op as u32);
            }
            Instruction::cdv => {
                let combo_op = get_value(operand, a, b, c);
                c = a / 2_i64.pow(combo_op as u32);
            }
        }

        if *instruction == Instruction::jnz && a != 0{
        }
        else {
            instruction_pointer += 1;
        }        
    }

    out

}

fn part2(instructions: &[Instruction], operands: &[i64], mut b: i64, mut c: i64) -> i64
{
    // let mut base_a = 8_i64.pow(instructions.len() as u32 * 2 - 1) + 450;
    let mut base_a = 0;
    let combined_program = instructions.iter().zip(operands).map(|(i, o)| {
        [*i as i64, *o]
    }).flatten().collect::<Vec<i64>>();


    
    for combined in (0..combined_program.len()).rev() {
        let matching_range = &combined_program[combined..];

        let mut a = base_a;
        loop {
            let test = part1(instructions, operands, a, b, c);
            if matching_range == test {
                base_a = a * 8;
                break;
            }
            a += 1;
        }
    }

    base_a / 8
}

fn main() {
    let input = include_str!("../input.txt");

    let mut line_it = input.split("\n\n");

    let mut register_values = line_it.next().unwrap().lines().map(|line| {
        line.split(": ").nth(1).unwrap().parse::<i64>().unwrap()
    });

    let a = register_values.next().unwrap();
    let b = register_values.next().unwrap();
    let c = register_values.next().unwrap();

    let inst_it = line_it.next().unwrap().split(": ").nth(1).unwrap().split(",").map(|x| x.parse::<i64>().unwrap());
    
    let op_it = inst_it.clone();

    let instructions = inst_it.step_by(2).map(|x| {
        unsafe {
            std::mem::transmute::<i64, Instruction>(x)
        }
    }).collect::<Vec<Instruction>>();

    let operands = op_it.skip(1).step_by(2).collect::<Vec<i64>>();

    part1(&instructions, &operands, a, b, c).iter().for_each(|x| print!("{},", x));
    println!();
    println!("{}", part2(&instructions, &operands, b, c));
}
