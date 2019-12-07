use std::fs;
use std::io::{self, Write};

fn run_program(mut program: Vec<i32>) -> i32 {    
    let mut i: usize = 0;

    while i < program.len() {
        let instr = program[i];
        let opcode = read_opcode(instr);
        i+= 1;

        if opcode == 99 {
            println!("Program exited");
            break;
        }

        if opcode == 1 || opcode == 2 || opcode == 7 || opcode == 8 {
            let param1 = read_param_value(i, &program, 0);
            let param2 = read_param_value(i, &program, 1);
            let result_address = program[i + 2] as usize;
            i += 3;
            if opcode == 1 {
                program[result_address] = param1 + param2;
            } else if opcode == 2 {
                program[result_address] = param1 * param2;
            } else if opcode == 7 {
                program[result_address] = if param1 < param2 { 1 } else { 0 };
            } else if opcode == 8 {
                program[result_address] = if param1 == param2 { 1 } else { 0 };
            }
        } else if opcode == 3 {
            let param = program[i];
            i += 1;
            let mut input_text = String::new();
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input_text).unwrap();
            let input_value = input_text
                .trim()
                .parse::<i32>()
                .expect("Input value is not an integer");
            program[param as usize] = input_value;
        } else if opcode == 4 {
            let param = read_param_value(i, &program, 0);
            i += 1;
            println!("{}", param);
        } else if opcode == 5 || opcode == 6 {
            let param1 = read_param_value(i, &program, 0);
            let param2 = read_param_value(i, &program, 1);
            i += 2;
            if (opcode == 5 && param1 != 0) || (opcode == 6 && param1 == 0) {
                i = param2 as usize;
            }
        }
    }
    
    fn read_opcode(instr: i32) -> i32 {
        return instr % 100;
    }
    
    fn read_param_mode(instr: i32, index: u32) -> i32 {
        return instr % 10_i32.pow(index + 3) / 10_i32.pow(index + 2);
    }
    
    fn read_param_value(start: usize, program: &Vec<i32>, index: u32) -> i32 {
        let mode = read_param_mode(program[start - 1], index);
        let param = program[start + index as usize];
        if mode == 0 {
            return program[param as usize];
        } else {
            return param;
        }
    }

    return program[0];
}

fn main() {
    let program = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    run_program(program.clone());
}
