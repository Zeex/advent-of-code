use std::cmp;
use std::fs;
use std::collections::{HashMap, VecDeque};

const TILE_WALL: u8 = 1;
const TILE_BLOCK: u8 = 2;
const TILE_PADDLE: u8 = 3;
const TILE_BALL: u8 = 4;

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

const EXIT_ERROR: i64 = -1;
const EXIT_HALT: i64 = 0;
const EXIT_NEED_INPUT: i64 = 1;
const EXIT_OUTPUT: i64 = 2;

struct Program {
    code: Vec<i64>,
    ip: i64,
    memory: HashMap::<i64, i64>,
    rel_base: i64
}

impl Clone for Program {
    fn clone(&self) -> Program {
        Program {
            code: self.code.clone(),
            ip: self.ip.clone(),
            memory: self.memory.clone(),
            rel_base: self.rel_base
        }
    }
}

fn init_program(program: &mut Program) {
    let code = &mut program.code;
    let memory = &mut program.memory;

    for i in 0..code.len() {
        memory.insert(i as i64, code[i]);
    }
}

fn run_program(program: &mut Program, data: &mut VecDeque<i64>) -> i64 {
    let code = &mut program.code;
    let memory = &mut program.memory;
    let mut ip = program.ip;
    let mut rel_base = program.rel_base;

    while ip < code.len() as i64 {
        let instr = memory[&ip];
        let opcode = read_opcode(instr);
        ip += 1;

        match opcode {
            OP_HALT => {
                return EXIT_HALT;
            },
            OP_ADD | OP_MULTIPLY | OP_LESS_THAN | OP_EQUALS => {
                let param1 = read_param_value(ip, &memory, rel_base, 0);
                let param2 = read_param_value(ip, &memory, rel_base, 1);
                let result_address = read_param_value_out(ip, &memory, rel_base, 2);
                ip += 3;
                match opcode {
                    OP_ADD => {
                        memory.insert(result_address, param1 + param2);
                    },
                    OP_MULTIPLY => {
                        memory.insert(result_address, param1 * param2);
                    },
                    OP_LESS_THAN => {
                        memory.insert(result_address, if param1 < param2 {1} else {0});
                    },
                    OP_EQUALS => {
                        memory.insert(result_address, if param1 == param2 {1} else {0});
                    }
                    _ => panic!()
                }
            },
            OP_INPUT => {
                let param = read_param_value_out(ip, &memory, rel_base, 0);
                ip += 1;
                match data.pop_front() {
                    Some(value) => {
                        memory.insert(param, value);
                    },
                    None => return EXIT_NEED_INPUT
                }
            },
            OP_OUTPUT => {
                let param = read_param_value(ip, &memory, rel_base, 0);
                ip += 1;
                data.push_back(param);
                program.ip = ip;
                return EXIT_OUTPUT;
            },
            OP_JUMP_IF_TRUE | OP_JUMP_IF_FALSE => {
                let param1 = read_param_value(ip, &memory, rel_base, 0);
                let param2 = read_param_value(ip, &memory, rel_base, 1);
                ip += 2;
                if (opcode == OP_JUMP_IF_TRUE && param1 != 0)
                        || (opcode == OP_JUMP_IF_FALSE && param1 == 0) {
                    ip = param2;
                }
            },
            OP_REL_BASE_OFFSET => {
                rel_base += read_param_value(ip, &memory, rel_base, 0);
                ip += 1;
            }
            _ => {
                println!("Invalid opcoe at address {}: {}", ip - 1, instr);
                return EXIT_ERROR;
            }
        }

        program.ip = ip;
        program.rel_base = rel_base;
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
            MODE_REL => {
                if rel_base + param < 0 {
                    panic!("Attempt to read memory at invalid address {}", rel_base + param);
                }
                *memory.get(&(param + rel_base)).unwrap_or(&0)
            }
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

    return EXIT_ERROR;
}

fn read_tiles(program: &mut Program) -> Vec<Vec<u8>> {
    let mut grid_map: HashMap<(i64, i64), u8> = HashMap::new();
    let mut data = VecDeque::new();

    init_program(program);

    loop {
        if run_program(program, &mut data) == EXIT_HALT { break; }
        let x = data.pop_front().unwrap();
        
        if run_program(program, &mut data) == EXIT_HALT { break; }
        let y = data.pop_front().unwrap();
        
        if run_program(program, &mut data) == EXIT_HALT { break; }
        let tile_id = data.pop_front().unwrap();
        
        grid_map.insert((x, y), tile_id as u8);
    }
    
    let max_x = grid_map.keys().map(|(x ,_)| x).fold(0, |max, &x| cmp::max(max, x));
    let max_y = grid_map.keys().map(|(_ ,y)| y).fold(0, |max, &y| cmp::max(max, y));
    let mut grid: Vec<Vec<u8>> = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    
    for ((x, y), tile_id) in grid_map {
        grid[y as usize][x as usize] = tile_id;
    }
    
    return grid;
}

fn draw_tiles(grid: &Vec<Vec<u8>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let tile = grid[i][j] as u8;
            let symbol = match tile {
                TILE_WALL => '#',
                TILE_BLOCK => '*',
                TILE_PADDLE => '_',
                TILE_BALL => 'o',
                _ => ' '
            };
            print!("{}", symbol);
        }
        print!("\n");
    }
}

fn count_block_tiles(grid: &Vec<Vec<u8>>) -> u64 {
    return grid.iter()
        .map(|line| line.iter().filter(|&&tile| tile == TILE_BLOCK).count() as u64)
        .fold(0, |sum, count| sum + count);
}

fn main() {
    let code = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut program = Program {code: code, ip: 0, memory: HashMap::new(), rel_base: 0};

    let mut grid = read_tiles(&mut program);
    draw_tiles(&mut grid);
    
    println!("Number of block tiles: {}", count_block_tiles(&grid));
}
