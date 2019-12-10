use std::fs;
use std::cmp;
use std::collections::VecDeque;

const OP_EXIT: i32 = 99;
const OP_ADD: i32 = 1;
const OP_MULTIPLY: i32 = 2;
const OP_INPUT: i32 = 3;
const OP_OUTPUT: i32 = 4;
const OP_JUMP_IF_TRUE: i32 = 5;
const OP_JUMP_IF_FALSE: i32 = 6;
const OP_LESS_THAN: i32 = 7;
const OP_EQUALS: i32 = 8;

fn run_program(program: &mut Vec<i32>, inputs: &mut VecDeque<i32>, outputs: &mut VecDeque<i32>) {
    let mut i: usize = 0;

    while i < program.len() {
        let instr = program[i];
        let opcode = read_opcode(instr);
        i+= 1;

        match opcode {
            OP_EXIT => {
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
                program[param as usize] = inputs.pop_front().unwrap();
            },
            OP_OUTPUT => {
                let param = read_param_value(i, &program, 0);
                i += 1;
                outputs.push_back(param);
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
}

fn permutations(values: Vec<i32>) -> Vec<Vec<i32>> {
    let mut v = values.clone();
    let mut ps = Vec::new();
    permutations_internal(&mut v, &mut ps, values.len());
    
    fn permutations_internal(v: &mut Vec<i32>, mut ps: &mut Vec<Vec<i32>>, i: usize) {
        if i == 1 {
            ps.push(v.clone());
            return;
        }
        permutations_internal(v, &mut ps, i - 1);
        for j in 0..(i - 1) {
            if i % 2 == 0 {
                v.swap(j, i - 1);
            } else {
                v.swap(0, i - 1);
            }
            permutations_internal(v, &mut ps, i - 1);
        }
    }
    
    return ps;
}

fn main() {
    let program = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut max_signal = 0;
    
    for phases in permutations((0..=4).collect()) {
        let mut amplifiers: Vec<Vec<i32>> = vec![program.clone(); phases.len()];
        let mut signal = 0;
        
        for (i, amp) in amplifiers.iter_mut().enumerate() {
            let mut inputs = VecDeque::new();
            inputs.push_back(phases[i]);
            inputs.push_back(signal);
            let mut outputs = VecDeque::new();
            run_program(amp, &mut inputs, &mut outputs);
            signal = outputs[0];
        }
        
        max_signal = cmp::max(max_signal, signal);
    }
    
    println!("{}", max_signal);
}
