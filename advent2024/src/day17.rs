use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

#[derive(Clone)]
struct State {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: Vec<usize>
}
pub async fn advent_1(data: String, _: bool) -> String {
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

    match opcode {
        0 => {
            let power = combo(operand, state) as u32;
            let temp = state.reg_a / (2 as u64).pow(power) as usize;
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
            state.reg_b = temp;
        },
        7 => {
            let power = combo(operand, state) as u32;
            let temp = state.reg_a / (2 as u64).pow(power) as usize;
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

pub async fn advent_2(data: String, _: bool) -> usize {
    let state = parse_state(&data);
    solve(&state.program)
}

fn solve(targets: &Vec<usize>) -> usize {
    let mut map: HashMap<usize, HashMap<usize, Vec<usize>>> = HashMap::new();
    for i in 0..(2<<10) {
        let target = i % 8;
        let output = single_iteration(i);
        let unshift = i >> 3;
        if let Some(submap) = map.get_mut(&unshift) {
            if let Some(vec) = submap.get_mut(&output) {
                vec.push(target);
            }
            else {
                let vec = vec![target];
                submap.insert(output, vec);
            }
        }
        else {
            let mut submap = HashMap::new();
            let vec = vec![target];
            submap.insert(output, vec);
            map.insert(unshift, submap);
        }
    }
    return solve_recursive(0, &targets, targets.len() - 1, &map).unwrap();
}

fn solve_recursive(a: usize, targets: &Vec<usize>, target_index: usize, map: &HashMap<usize, HashMap<usize, Vec<usize>>>) -> Option<usize> {
    let shifted = a << 3;
    let key = a % (2 << 7);
    let target = targets[target_index];
    // println!("for target {target} and a {a} and shifted {shifted} and key {key}");
    let submap = map.get(&key).unwrap();
    if let Some(vec) = submap.get(&target) {
        if target_index == 0 {
            let least = vec.iter().fold(usize::MAX, |a,b| std::cmp::min(a,*b));
            return Some(shifted + least);
        }
        for digit in vec {
            let next = shifted + digit;
            // println!("{}",format!("{next:b} aka {next}"));
            if let Some(ret) = solve_recursive(next, targets, target_index - 1, &map) {
                return Some(ret);
            }
        }
    }

    None
}

fn single_iteration(start: usize) -> usize {
    let a = start;
    let mut b;
    let c;
    b = a % 8;
    b = b ^ 2;
    c = a >> b;
    // a = a >> 3;
    b = b ^ 7;
    b = b ^ c;
    return b % 8;
}
