// Convert Intcode programs into a human readable assembly-like language.

use std::env;
use std::fs;
use std::process;
use std::collections::HashMap;

const MODE_POS: i64 = 0;
const MODE_IMM: i64 = 1;
const MODE_REL: i64 = 2;

const OP_HALT: i64 = 99;
const OP_ADD: i64 = 1;
const OP_MULTIPLY: i64 = 2;
const OP_INPUT: i64 = 3;
const OP_OUTPUT: i64 = 4;
const OP_JUMP_IF_TRUE: i64 = 5;
const OP_JUMP_IF_FALSE: i64 = 6;
const OP_LESS_THAN: i64 = 7;
const OP_EQUALS: i64 = 8;
const OP_REL_BASE_OFFSET: i64 = 9;

fn print_program(program: &Vec<i64>) {
    let mut ip: i64 = 0;

    let opcode_names: HashMap<i64, &str> = [
        (OP_HALT, "halt"),
        (OP_ADD, "add"),
        (OP_MULTIPLY, "multiply"),
        (OP_INPUT, "input"),
        (OP_OUTPUT, "output"),
        (OP_JUMP_IF_TRUE, "jump_if_true"),
        (OP_JUMP_IF_FALSE, "jump_if_false"),
        (OP_LESS_THAN, "less_than"),
        (OP_EQUALS, "equals"),
        (OP_REL_BASE_OFFSET, "rel_base_offset")
    ].iter().cloned().collect();

    while ip < program.len() as i64 {
        let instr = program[ip as usize];
        let opcode = read_opcode(instr);

        match opcode_names.get(&opcode) {
            Some(name) => print!("{:0>4}: {: >16}   ", ip, name),
            None => print!("{:0>4}: {: >16}   ", ip, opcode)
        }

        ip += 1;
        
        let param_count: i64 = match opcode {
            OP_ADD | OP_MULTIPLY | OP_LESS_THAN | OP_EQUALS => 3,
            OP_JUMP_IF_TRUE | OP_JUMP_IF_FALSE => 2,
            OP_INPUT | OP_OUTPUT | OP_REL_BASE_OFFSET => 1,
            _ => 0
        };

        print_params(program, ip, param_count as usize);
        print!("\n");
        ip += param_count;
    }

    fn read_opcode(instr: i64) -> i64 {
        instr % 100
    }
    
    fn read_param_mode(instr: i64, index: usize) -> i64 {
        instr % 10_i64.pow((index + 3) as u32) / 10_i64.pow((index + 2) as u32)
    }
    
    fn print_params(program: &Vec<i64>, start: i64, count: usize) {
        for i in 0..count {
            let mode = read_param_mode(program[(start - 1) as usize], i);
            let param = program[start as usize + i];
            match mode {
                MODE_POS => print!(" [{}]", param),
                MODE_IMM => print!(" {}", param),
                MODE_REL => print!(" [${}]", param),
                _ => {}
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <program_path>", args[0]);
        process::exit(1);
    }

    let program = fs::read_to_string(&args[1])
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    print_program(&program);
}