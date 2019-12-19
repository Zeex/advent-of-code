extern crate pancurses;

use std::fs;
use std::collections::{HashMap, VecDeque};
use std::thread;
use std::time;
use pancurses::{Window, Input};

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
    rel_base: i64,
    data: VecDeque<i64>
}

impl Program {
    pub fn new(code: Vec<i64>) -> Program {
        Program {
            code: code,
            ip: 0,
            memory: HashMap::new(),
            rel_base: 0,
            data: VecDeque::new()
        }
    }
}

impl Clone for Program {
    fn clone(&self) -> Program {
        Program {
            code: self.code.clone(),
            ip: self.ip.clone(),
            memory: self.memory.clone(),
            rel_base: self.rel_base,
            data: self.data.clone()
        }
    }
}

fn load_program(path: &str) -> Program {
    let code = fs::read_to_string(path)
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    return Program::new(code);
}

fn init_program(program: &mut Program) {
    program.ip = 0;
    program.memory.clear();
    program.rel_base = 0;
    program.data.clear();

    for i in 0..program.code.len() {
        program.memory.insert(i as i64, program.code[i]);
    }
}

fn run_program(program: &mut Program) -> i64 {
    let code = &mut program.code;
    let memory = &mut program.memory;
    let mut ip = program.ip;
    let mut rel_base = program.rel_base;
    let data = &mut program.data;

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

// struct GameState {
//     grid: Vec<Vec<u8>>,
//     score: i64,
//     steps: Vec<i32>,
//     steps_to_replay: VecDeque<i32>
// }

// impl GameState {
//     pub fn new() -> GameState {
//         GameState {
//             grid: vec![vec![0; 80]; 60],
//             score: 0,
//             steps: Vec::new(),
//             steps_to_replay: VecDeque::new()
//         }
//     }
// }

// fn run_game(program: &mut Program, game: &mut GameState, window: &Window) {
//     init_program(program);
//     program.memory.insert(0, 2); // insert 2 quarters

//     window.refresh();
//     window.keypad(true);

//     'game: loop {
//         match run_program(program) {
//             EXIT_NEED_INPUT => {
//                 window.clear();
//                 window.printw(format!("Score: {}\n\n", game.score));

//                 for i in 0..game.grid.len() {
//                     for j in 0..game.grid[i].len() {
//                         let tile = game.grid[i][j] as u8;
//                         let symbol = match tile {
//                             TILE_WALL => "#",
//                             TILE_BLOCK => "*",
//                             TILE_PADDLE => "_",
//                             TILE_BALL => "o",
//                             _ => " "
//                         };
//                         window.printw(symbol);
//                     }
//                     window.printw("\n");
//                 }

//                 match game.steps_to_replay.pop_front() {
//                     Some(v) => {
//                         program.data.push_back(v as i64);
//                         continue;
//                     }
//                     None => {}
//                 }

//                 loop {
//                     let key = window.getch();
//                     match key {
//                         Some(Input::KeyLeft) => {
//                             program.data.push_back(-1);
//                             break;
//                         },
//                         Some(Input::KeyRight) => {
//                             program.data.push_back(1);
//                             break;
//                         },
//                         Some(Input::Character(' ')) => {
//                             program.data.push_back(0);
//                             break;
//                         },
//                         Some(Input::Character('l')) => {
//                             game.steps = fs::read_to_string("save.txt").unwrap()
//                                 .split(",")
//                                 .map(|x| x.parse::<i32>().unwrap())
//                                 .collect();
//                             game.steps_to_replay = game.steps.clone().into_iter().collect();
//                             continue 'game;
//                         },
//                         Some(Input::Character('s')) => {
//                             fs::write("save.txt",
//                                 game.steps.iter()
//                                     .map(|x| x.to_string())
//                                     .collect::<Vec<String>>().join(",")).unwrap();
//                             break;
//                         },
//                         Some(Input::Character('q')) => {
//                             println!("Goodbye!");
//                             return;
//                         },
//                         _ => {}
//                     }
//                 }

//                 if !program.data.is_empty() {
//                     game.steps.push(*program.data.back().unwrap() as i32);
//                 }
//             },
//             EXIT_OUTPUT => {
//                 if program.data.len() == 3 {
//                     let x = program.data.pop_front().unwrap();
//                     let y = program.data.pop_front().unwrap();
//                     let tile = program.data.pop_front().unwrap();
//                     match (x, y) {
//                         (-1, 0) => {
//                             if tile != 0 {
//                                 game.score = tile;
//                             }
//                         },
//                         _ => {
//                             game.grid[y as usize][x as usize] = tile as u8;
//                         }
//                     }
//                 }
//             },
//             EXIT_HALT => {
//                 break;
//             }
//             _ => {
//                 println!("Oops, something went wrong");
//                 break;
//             }
//         }
//     }
// }

fn run_game_auto(program: &mut Program, window: &Window) {
    init_program(program);
    program.memory.insert(0, 2); // insert 2 quarters

    let mut grid: Vec<Vec<u8>> = vec![vec![0; 50]; 25];

    let mut score: i64 = 0;
    // let mut ball_pos = (0, 0);
    // let mut prev_ball_pos = (0, 0);
    // let mut player_pos = (0, 0);

    loop {
        // let (ball_x, ball_y) = ball_pos;
        // let (prev_ball_x, prev_ball_y) = prev_ball_pos;
        // let (player_x, player_y) = player_pos;
        // let next_ball_x = if ball_y < prev_ball_y {
        //     prev_ball_x + (prev_ball_y - player_y) * (ball_x - prev_ball_x) / (prev_ball_y - ball_y)
        // } else {
        //     ball_x
        // };

        match run_program(program) {
            EXIT_NEED_INPUT => {
                // for i in 0..grid.len() {
                //     for j in 0..grid[i].len() {
                //         let p = (j as i32, (grid.len() - i - 1) as i32);
                //         match tile {
                //             TILE_BALL => {
                //                 if ball_pos != p {
                //                     prev_ball_pos = ball_pos;
                //                     ball_pos = p;
                //                 }
                //             },
                //             TILE_PADDLE => {
                //                 player_pos = p;
                //             },
                //             _ => {}
                //         }
                //     }
                // }

                // thread::sleep(time::Duration::from_millis(5));

                // if next_ball_x < player_x {
                //     program.data.push_back(-2);
                // } else if next_ball_x > player_x {
                //     program.data.push_back(2);
                // } else {
                //     program.data.push_back(0);
                // }

                program.data.push_back(0);
            },
            EXIT_OUTPUT => {
                if program.data.len() == 3 {
                    let x = program.data.pop_front().unwrap();
                    let y = program.data.pop_front().unwrap();
                    let tile = program.data.pop_front().unwrap();
                    match (x, y) {
                        (-1, 0) => score = tile,
                        _ => grid[y as usize][x as usize] = tile as u8
                    }

                    window.clear();
                    window.printw(format!("Score: {}\n", score));
                    // window.printw(format!("Ball @ {:?}\n", ball_pos));
                    // window.printw(format!("Player @ {:?}\n", player_pos));
                    window.printw("\n");

                    for i in 0..grid.len() {
                        for j in 0..grid[i].len() {
                            let tile = grid[i][j] as u8;
                            let symbol = match tile {
                                TILE_WALL => "#",
                                TILE_BLOCK => "*",
                                TILE_PADDLE => "_",
                                TILE_BALL => "o",
                                _ => " "
                            };
                            window.printw(symbol);
                            // let p = (j as i32, (grid.len() - i - 1) as i32);
                            // match tile {
                            //     TILE_BALL => {
                            //         if ball_pos != p {
                            //             prev_ball_pos = ball_pos;
                            //             ball_pos = p;
                            //         }
                            //     },
                            //     TILE_PADDLE => {
                            //         player_pos = p;
                            //     },
                            //     _ => {}
                            // }
                        }
                        window.printw("\n");
                    }
                    window.refresh();
                }
            },
            EXIT_HALT => {
                window.getch();
                break;
            }
            _ => {
                println!("Oops, something went wrong");
                break;
            }
        }
    }
}

fn main() {
    let window = pancurses::initscr();
    pancurses::noecho();
    pancurses::cbreak();

    let mut program = load_program("input.txt");
    // let mut game = GameState::new();
    // run_game(&mut program, &mut game, &window);
    // println!("Your score was: {}", game.score);

    run_game_auto(&mut program, &window);

    // pancurses::endwin();
}
