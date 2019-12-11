use std::fs;
use std::collections::VecDeque;

const OP_HALT: i32 = 99;
const OP_ADD: i32 = 1;
const OP_MULTIPLY: i32 = 2;
const OP_INPUT: i32 = 3;
const OP_OUTPUT: i32 = 4;
const OP_JUMP_IF_TRUE: i32 = 5;
const OP_JUMP_IF_FALSE: i32 = 6;
const OP_LESS_THAN: i32 = 7;
const OP_EQUALS: i32 = 8;

const EXIT_END: i32 = -1;
const EXIT_HALT: i32 = 0;
const EXIT_NEED_INPUT: i32 = 1;
const EXIT_OUTPUT: i32 = 2;

struct Program {
    code: Vec<i32>,
    ip: usize
}

impl Clone for Program {
    fn clone(&self) -> Program {
        Program {code: self.code.clone(), ip: self.ip.clone()}
    }
}

fn run_program(program: &mut Program, data: &mut VecDeque<i32>) -> i32 {
    let mut ip = program.ip as usize;
    let code = &mut program.code;

    while ip < code.len() {
        let instr = code[ip];
        let opcode = read_opcode(instr);
        ip += 1;

        match opcode {
            OP_HALT => {
                return EXIT_HALT;
            },
            OP_ADD | OP_MULTIPLY | OP_LESS_THAN | OP_EQUALS => {
                let param1 = read_param_value(ip, &code, 0);
                let param2 = read_param_value(ip, &code, 1);
                let result_address = code[ip + 2] as usize;
                ip += 3;
                match opcode {
                    OP_ADD => code[result_address] = param1 + param2,
                    OP_MULTIPLY => code[result_address] = param1 * param2,
                    OP_LESS_THAN => code[result_address] = if param1 < param2 { 1 } else { 0 },
                    OP_EQUALS => code[result_address] = if param1 == param2 { 1 } else { 0 },
                    _ => panic!()
                }
            },
            OP_INPUT => {
                let param = code[ip];
                ip += 1;
                match data.pop_front() {
                    Some(value) => code[param as usize] = value,
                    None => return EXIT_NEED_INPUT
                }
            },
            OP_OUTPUT => {
                let param = read_param_value(ip, &code, 0);
                ip += 1;
                data.push_back(param);
                program.ip = ip;
                return EXIT_OUTPUT;
            },
            OP_JUMP_IF_TRUE | OP_JUMP_IF_FALSE => {
                let param1 = read_param_value(ip, &code, 0);
                let param2 = read_param_value(ip, &code, 1);
                ip += 2;
                if (opcode == OP_JUMP_IF_TRUE && param1 != 0) || (opcode == OP_JUMP_IF_FALSE && param1 == 0) {
                    ip = param2 as usize;
                }
            }
            _ => panic!()

        }

        program.ip = ip;
    }

    fn read_opcode(instr: i32) -> i32 {
        return instr % 100;
    }

    fn read_param_mode(instr: i32, index: u32) -> i32 {
        return instr % 10_i32.pow(index + 3) / 10_i32.pow(index + 2);
    }

    fn read_param_value(start: usize, code: &Vec<i32>, index: u32) -> i32 {
        let mode = read_param_mode(code[start - 1], index);
        let param = code[start + index as usize];
        if mode == 0 {
            return code[param as usize];
        } else {
            return param;
        }
    }
    
    return EXIT_END;
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
    let code = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let program = Program {code: code, ip: 0};
    let mut max_signal = 0;
    let mut max_signal_phases = vec!();

    for phases in permutations((5..=9).collect()) {
        let mut amplifiers: Vec<Program> = vec![program.clone(); phases.len()];
        let mut signal = 0;
        let mut data = VecDeque::new();

        println!("Phases: {:?}", phases);
        
        for (index, amp) in amplifiers.iter_mut().enumerate() {
            println!("Initializing amplifier {}", index);
            data.push_back(phases[index]);
            if run_program(amp, &mut data) != EXIT_NEED_INPUT {
                panic!();
            }
        }
        
        data.push_back(signal);
        
        loop {
            let mut exit_code = EXIT_HALT;
            
            for (index, amp) in amplifiers.iter_mut().enumerate() {
                println!("Running amplifier {} with input signal {}", index, data[0]);
                exit_code = run_program(amp, &mut data);
                println!("Output signal: {}", data[0]);
            }
            
            if exit_code == EXIT_HALT {
                signal = data[0];
                break;
            }
        }

        if signal > max_signal {
            max_signal = signal;
            max_signal_phases = phases;   
        }
    }

    println!("{}", max_signal);
    println!("{:?}", max_signal_phases);
}
