use std::fs;

fn run_program(mut program: Vec<u32>, arg1: u32, arg2: u32) -> u32 {
    program[1] = arg1;
    program[2] = arg2;

    let mut i: usize = 0;

    while i < program.len() {
        let opcode = program[i];
        i+= 1;

        if opcode == 99 {
            break;
        }

        if opcode == 1 || opcode == 2 {
            let op1 = program[program[i] as usize];
            let op2 = program[program[i + 1] as usize];
            let result_pos = program[i + 2] as usize;
            i += 3;

            if opcode == 1 {
                program[result_pos] = op1 + op2;
            } else if opcode == 2 {
                program[result_pos] = op1 * op2;
            }
        }
    }

    return program[0];
}

fn main() {
    let program = fs::read_to_string("input_1.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    for i in 1..program.len() {
        for j in 1..program.len() {
            if run_program(program.clone(), i as u32, j as u32) == 19690720 {
                println!("{} {} {}", i, j, i * 100 + j);
            }        
        }
    }
}
