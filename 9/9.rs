use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;

const MODE_POS: i64 = 0;
const MODE_IMM: i64 = 1;
const MODE_REL: i64 = 2;

const OP_EXIT: i64 = 99;
const OP_ADD: i64 = 1;
const OP_MULTIPLY: i64 = 2;
const OP_INPUT: i64 = 3;
const OP_OUTPUT: i64 = 4;
const OP_JUMP_IF_TRUE: i64 = 5;
const OP_JUMP_IF_FALSE: i64 = 6;
const OP_LESS_THAN: i64 = 7;
const OP_EQUALS: i64 = 8;
const OP_REL_BASE_OFFSET: i64 = 9;

fn run_program(program: &Vec<i64>) {
    let mut memory = HashMap::<i64, i64>::new();

    for i in 0..program.len() {
        memory.insert(i as i64, program[i]);
    }
    
    let mut address: i64 = 0;
    let mut rel_base: i64 = 0;
    
    while address < program.len() as i64 {
        let instr = memory[&address];
        let opcode = read_opcode(instr);
        address += 1;

        match opcode {
            OP_EXIT => {
                println!("Program exited");
                break;
            },
            OP_ADD | OP_MULTIPLY | OP_LESS_THAN | OP_EQUALS => {
                let param1 = read_param_value(address, &memory, rel_base, 0);
                let param2 = read_param_value(address, &memory, rel_base, 1);
                let result_address = read_param_value_out(address, &memory, rel_base, 2);
                address += 3;
                match opcode {
                    OP_ADD => {
                        memory.insert(result_address, param1 + param2);
                    },
                    OP_MULTIPLY => {
                        memory.insert(result_address, param1 * param2);
                    },
                    OP_LESS_THAN => {
                        memory.insert(result_address, if param1 < param2 { 1 } else { 0 });
                    },
                    OP_EQUALS => {
                        memory.insert(result_address, if param1 == param2 { 1 } else { 0 });
                    }
                    _ => panic!()
                }
            },
            OP_INPUT => {
                let mut input_text = String::new();
                print!("> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input_text).unwrap();
                let input_value = input_text
                    .trim()
                    .parse::<i64>()
                    .expect("Input value is not an integer");
                let result_address = read_param_value_out(address, &memory, rel_base, 0);
                memory.insert(result_address, input_value);
                address += 1;
            },
            OP_OUTPUT => {
                let param = read_param_value(address, &memory, rel_base, 0);
                address += 1;
                println!("{}", param);
            },
            OP_JUMP_IF_TRUE | OP_JUMP_IF_FALSE => {
                let param1 = read_param_value(address, &memory, rel_base, 0);
                let param2 = read_param_value(address, &memory, rel_base, 1);
                address += 2;
                if (opcode == OP_JUMP_IF_TRUE && param1 != 0) 
                        || (opcode == OP_JUMP_IF_FALSE && param1 == 0) {
                    address = param2;
                }
            },
            OP_REL_BASE_OFFSET => {
                rel_base += read_param_value(address, &memory, rel_base, 0);
                address += 1;
            }
            _ => {}
        }
    }

    fn read_opcode(instr: i64) -> i64 {
        instr % 100
    }

    fn read_param_mode(instr: i64, index: u32) -> i64 {
        instr % 10_i64.pow(index + 3) / 10_i64.pow(index + 2)
    }
    
    fn read_param_value(start: i64, memory: &HashMap::<i64, i64>, rel_base: i64, index: u32) -> i64 {
        let mode = read_param_mode(memory[&(start - 1)], index);
        let param = *memory.get(&(start + index as i64)).unwrap_or(&0);
        match mode {
            MODE_POS => *memory.get(&param).unwrap_or(&0),
            MODE_IMM => param,
            MODE_REL => *memory.get(&(param + rel_base)).unwrap_or(&0),
            _ => panic!()
        }
    }
    
    fn read_param_value_out(start: i64, memory: &HashMap::<i64, i64>, rel_base: i64, index: u32) -> i64 {
        let mode = read_param_mode(memory[&(start - 1)], index);
        let param = *memory.get(&(start + index as i64)).unwrap_or(&0);
        match mode {
            MODE_POS => param,
            MODE_REL => param + rel_base,
            _ => panic!()
        }
    }
}

fn main() {
    let program = fs::read_to_string("9_input.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    run_program(&program);
}
