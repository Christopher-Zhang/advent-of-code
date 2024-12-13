use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use crate::util::*;


pub async fn advent_1(data: String) -> usize {
    let mut stones = data.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect_vec();
    _print_stones(&stones);
    let mut last = 1;
    for iteration in 0.. 6{
        stones = blink(stones);
        // print!("{iteration}: ");
        _print_stones(&stones);
        println!();
        // let cur = stones.len();
        // println!("{}", cur - last);
        // last = cur;
        // println!("{}", stones.len());
    }
    stones.len()
}
fn _print_stones (stones: &Vec<usize>) {
    let str = stones.iter().join(" ");
    println!("{str}");
}

fn blink(stones: Vec<usize>) -> Vec<usize> {
    let mut result = vec![];
    for stone in stones {
        if stone == 0 {
            result.push(1);
        }
        else {
            let digits = count_digits(stone);
            if digits % 2 != 0 {
                result.push(stone * 2024);
            }
            else {
                let mut factor = (10 as usize).pow((digits/2) as u32);
                result.push(stone / factor);
                result.push(stone % factor);
            }
        }

    }
    result
}

fn count_digits(num: usize) -> usize {
    let mut digits = 0;
    let mut cur = num;
    while cur != 0 {
        cur /= 10;
        digits += 1;
    } 
    return digits;
}


pub async fn advent_2(data: String) -> usize {
    let stones: VecDeque<usize> = data.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect_vec().into();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    
    stones.iter().map(|stone| recursive(0, *stone, &mut cache)).sum()
}
fn recursive(blink_num: usize, stone: usize, cache: &mut HashMap<(usize,usize), usize>) -> usize {
    if blink_num == 75 {
        return 1;
    }
    if let Some(cached) = cache.get(&(blink_num, stone)) {
        return *cached;
    }
    else {
        let ret;
        if stone == 0 {
            ret = recursive(blink_num + 1, 1, cache);
        }
        else {
            let digits = count_digits(stone);
            let factor = (10 as usize).pow((digits/2) as u32);
            if digits % 2 == 0 {
                ret = recursive(blink_num + 1, stone / factor, cache) + recursive(blink_num + 1, stone % factor, cache);
            }
            else {
                ret = recursive(blink_num + 1, stone * 2024, cache);
            }
        }
        if stone / 100 == 0 {
            cache.insert((blink_num, stone), ret);
        }
        return ret;   
    }
}
fn brute(data: &str) -> usize {
    let mut stones: VecDeque<usize> = data.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect_vec().into();
    for iter in 0..75 {
        println!("{iter}");
        let len = stones.len();
        for _ in 0..len {
            let stone = stones[0];
            if stone == 0 {
                stones.push_back(1);
            }
            else {
                let digits = count_digits(stone);
                if digits % 2 != 0 {
                    stones.push_back(stone * 2024);
                }
                else {
                    let factor = (10 as usize).pow((digits/2) as u32);
                    stones.push_back(stone / factor);
                    stones.push_back(stone % factor);
                }
            }
            stones.pop_front();
        }
    }
    stones.len()
}
