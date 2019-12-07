use std::fs;
use std::io::{self, Write};

const OP_EXIT: i32 = 99;
const OP_ADD: i32 = 1;
const OP_MULTIPLY: i32 = 2;
const OP_INPUT: i32 = 3;
const OP_OUTPUT: i32 = 4;
const OP_JUMP_IF_TRUE: i32 = 5;
const OP_JUMP_IF_FALSE: i32 = 6;
const OP_LESS_THAN: i32 = 7;
const OP_EQUALS: i32 = 8;

fn run_program(mut program: Vec<i32>) -> i32 {    
    let mut i: usize = 0;

    while i < program.len() {
        let instr = program[i];
        let opcode = read_opcode(instr);
        i+= 1;

        match opcode {
            OP_EXIT => {
                println!("Program exited");
                break;
            },
            OP_ADD | OP_MULTIPLY | OP_LESS_THAN | OP_EQUALS => {
                let param1 = read_param_value(i, &program, 0);
                let param2 = read_param_value(i, &program, 1);
                let result_address = program[i + 2] as usize;
                i += 3;
                match opcode {
                    OP_ADD => program[result_address] = param1 + param2,
                    OP_MULTIPLY => program[result_address] = param1 * param2,
                    OP_LESS_THAN => program[result_address] = if param1 < param2 { 1 } else { 0 },
                    OP_EQUALS => program[result_address] = if param1 == param2 { 1 } else { 0 },
                    _ => {}
                }
            },
            OP_INPUT => {
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
            },
            OP_OUTPUT => {
                let param = read_param_value(i, &program, 0);
                i += 1;
                println!("{}", param);
            },
            OP_JUMP_IF_TRUE | OP_JUMP_IF_FALSE => {
                let param1 = read_param_value(i, &program, 0);
                let param2 = read_param_value(i, &program, 1);
                i += 2;
                if (opcode == OP_JUMP_IF_TRUE && param1 != 0) || (opcode == OP_JUMP_IF_FALSE && param1 == 0) {
                    i = param2 as usize;
                }
            }
            _ => {}
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
