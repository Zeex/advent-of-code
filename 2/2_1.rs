use std::fs;

fn main() {
    let mut program = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    program[1] = 12;
    program[2] = 2;

    // for x in &program {
    //     println!("{}", x);
    // }
    
    let mut i: usize = 0;
    
    while i < program.len() {
        let opcode = program[i];
        i+= 1;

        // println!("index = {}, opcode = {}", i, opcode);

        if opcode == 99 {
            break;
        }

        if opcode == 1 || opcode == 2 {
            let op1 = program[program[i] as usize];
            let op2 = program[program[i + 1] as usize];
            let result_pos = program[i + 2] as usize;
            i += 3;
            
            // println!("op1 = {}, op2 = {}, result_pos = {}", op1, op2, result_pos);
            
            if opcode == 1 {
                program[result_pos] = op1 + op2;
                println!("{} <- {} + {} = {}", result_pos, op1, op2, op1 + op2);
            } else if opcode == 2 {
                program[result_pos] = op1 * op2;
                println!("{} <- {} * {} = {}", result_pos, op1, op2, op1 * op2);
            }
        }
    }

    println!("Result: {}", program[0]);
}
