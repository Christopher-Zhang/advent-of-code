use std::collections::HashMap;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use crate::util::*;


pub async fn advent_1(data: String, _test: bool) -> usize {
    let machines = parse(data);

    machines.iter().enumerate().map(|(i, (display, buttons, _))| {
        let mut cache = HashMap::<String, isize>::new();
        let mut q = PriorityQueue::new();
        q.push((std::iter::repeat(".").take(display.len()).collect::<String>(), String::from("")), 0);
        while let Some(((cur_state, cur_path), cur_steps)) = q.pop() {
            let cur_steps = -cur_steps;

            if let Some(steps) = cache.get(&cur_state) && cur_steps >= *steps {
                continue;
            }
            if &cur_state == display {
                // println!("FOR LINE {i}, the fewest presses is {cur_steps} to get to {display} by pressing {cur_path}");
                return cur_steps as usize;
            }
            cache.insert(cur_state.clone(), cur_steps);
            for (i,button) in buttons.iter().enumerate() {
                let mut state = cur_state.chars().collect_vec();
                for index in button {
                    match state[*index] {
                        '#' => state[*index] = '.',
                        '.' => state[*index] = '#',
                        _ => panic!("panik!")
                    };
                }

                let state = state.iter().join("");
                if !cache.contains_key(&state) && !q.contains(&(state.clone(), format!(""))){
                    q.push((state, format!("{cur_path} {i}")), -(cur_steps + 1));                }
            }

        }
        0
    }).sum()
}

pub async fn advent_2(data: String, _test: bool) -> usize {
    let machines = parse(data);

    machines.iter().enumerate().map(|(i, (_, buttons, joltage))| {
        let mut cache = HashMap::<String, isize>::new();
        let mut q = PriorityQueue::new();
        q.push((vec![0;joltage.len()], String::from("")), 0);
        while let Some(((cur_state, cur_path), cur_steps)) = q.pop() {
            let cur_steps = -cur_steps;
            let cur_state_str = cur_state.iter().join("");
            if let Some(steps) = cache.get(&cur_state_str) && cur_steps >= *steps {
                continue;
            }
            if cur_state == *joltage {
                // println!("FOR LINE {i}, the fewest presses is {cur_steps} to get to {display} by pressing {cur_path}");
                return cur_steps as usize;
            }
            cache.insert(cur_state_str.clone(), cur_steps);
            for (i,button) in buttons.iter().enumerate() {
                let mut state = cur_state.clone();
                for index in button {
                    state[*index] += 1;
                }
                let state_str = state.iter().join("");

                if !cache.contains_key(&state_str) && !q.contains(&(state.clone(), format!(""))){
                    q.push((state, format!("{cur_path} {i}")), -(cur_steps + 1));                }
            }

        }
        0
    }).sum()
}

fn parse(data: String) -> Vec<(String, Vec<Vec<usize>>, Vec<usize>)> {
    let godforsaken_and_banned_brackets = ['[', ']', '(', ')', '{', '}'];
    let saved_and_angelic_data: String = data.chars().filter(|c| !godforsaken_and_banned_brackets.contains(c)).collect();
    saved_and_angelic_data.split("\n").map(|line| {
        
        let split = line.split(" ").collect_vec();
        let display = split[0];
        let joltages_str = split[split.len()-1];
        let joltages = joltages_str.split(",").map(|joltage| joltage.parse::<usize>().unwrap()).collect_vec();
        let buttons_str = &split[1..split.len()-1];

        let buttons = buttons_str.iter().map(|button| {
            button.split(",").map(|butt| butt.parse::<usize>().unwrap()).collect_vec()
        }).collect_vec();
        (display.to_string(), buttons, joltages)
    }).collect_vec()
}
