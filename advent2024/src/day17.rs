use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use crate::util::*;

#[derive(Clone)]
struct State {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: Vec<usize>
}
pub async fn advent_1(data: String) -> String {
    let mut state = parse_state(&data);
    let mut ip = 0;
    let mut out = Vec::<usize>::new();
    while ip < state.program.len() {
        let opcode = state.program[ip];
        let operand = state.program[ip+1];
        let result = operation(opcode, operand, &mut state);
        if let Some(new_ip) = result.1 {
            ip = new_ip;
        }
        else {
            ip += 2;
        }

        if let Some(output) = result.0 {
            out.push(output);
        }
    }
    out.iter().join(",")
}
fn operation(opcode: usize, operand: usize, state: &mut State) -> (Option<usize>, Option<usize>) {
    let mut ret = (None, None);

    let output = match opcode {
        0 => {
            let power = combo(operand, state) as u32;
            let temp = state.reg_a / (2 as u64).pow(power) as usize;
            // println!("division {opcode} {operand} results in {temp}");
            state.reg_a = temp;
        },
        1 => {
            state.reg_b = state.reg_b ^ operand;
        },
        2 => {
            let a = combo(operand, state);
            state.reg_b = a % 8;
        },
        3 => {
            if state.reg_a != 0 {
                ret.1 = Some(operand);
            }
        },
        4 => {
            state.reg_b = state.reg_b ^ state.reg_c;
        },
        5 => {
            let a = combo(operand, state);
            ret.0 = Some(a % 8);
        },
        6 => {
            let power = combo(operand, state) as u32;
            let temp = state.reg_a / (2 as u64).pow(power) as usize;
            // println!("division {opcode} {operand} results in {temp}");
            state.reg_b = temp;
        },
        7 => {
            let power = combo(operand, state) as u32;
            let temp = state.reg_a / (2 as u64).pow(power) as usize;
            // println!("division {opcode} {operand} results in {temp}");
            state.reg_c = temp;
        },
        _ => panic!("panik {opcode}")
    };
    ret
} 
fn combo (operand: usize, state: &State) -> usize {
    match operand {
        0 | 1 | 2 | 3 => operand,
        4 => state.reg_a,
        5 => state.reg_b,
        6 => state.reg_c,
        _ => panic!("panik operand {operand}")
    }
}
fn parse_state(data: &str) -> State {
    let split = data.split("\n\n").collect_vec();
    let re = Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)").unwrap();
    let caps = re.captures(split[0]).unwrap().iter().skip(1).map(|cap| cap.unwrap().as_str().parse::<usize>().unwrap()).collect_vec();
    let re = Regex::new(r"Program: (.+)").unwrap();
    let program = re.captures(split[1]).unwrap().iter().skip(1).collect_vec()[0].unwrap().as_str().split(",").map(|x| x.parse::<usize>().unwrap()).collect_vec();
    State { reg_a: caps[0], reg_b: caps[1], reg_c: caps[2], program }
}
pub async fn advent_2(data: String) -> usize {
    let mut ret = 0;
    let initial_state = parse_state(&data);
    for i in 235210000..10000000000 {
        if i % 1000000 == 0 {
            println!("iteration {i}");
        }
        let mut state = initial_state.clone();
        state.reg_a = i;
        let mut ip = 0;
        let mut out = Vec::<usize>::new();
        while ip < state.program.len() {
            let opcode = state.program[ip];
            let operand = state.program[ip+1];
            let result = operation(opcode, operand, &mut state);
            if let Some(new_ip) = result.1 {
                ip = new_ip;
            }
            else {
                ip += 2;
            }
    
            if let Some(output) = result.0 {
                out.push(output);
            }
        }
        if out.len() == initial_state.program.len() && initial_state.program.iter().enumerate().all(|(i,val)| *val == out[i]) {
            dbg!(initial_state.program);
            dbg!(out);
            ret = i;
            break;
        }
    }

    ret
}
