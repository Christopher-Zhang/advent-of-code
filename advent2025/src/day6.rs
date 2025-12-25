use std::collections::HashMap;
use itertools::Itertools;
use crate::util::*;


pub async fn advent_1(data: String, _test: bool) -> usize {
    let mut total = 0;

    total
}

pub async fn advent_2(data: String, _test: bool) -> usize {
    let mut total = 0;
    let (numbers, mut operations) = parse(&data);

    let mut find_next = true;
    let length = numbers[0].len();
    for i in (0..length).rev() {
        // println!("{i}: {}",numbers.iter().map(|line| line[i]).join(""));
        if find_next && numbers.iter().any(|c| c[i] != ' ') {
            find_next = false;
            let cur = compute(i, &numbers, &mut operations);
            total += cur;
        }
        else if !find_next {
            if numbers.iter().all(|c| c[i] == ' ') {
                find_next = true;
            }
        }
    }


    total
}

fn compute(col: usize, numbers: &Vec<Vec<char>>, operations: &mut Vec<&str>) -> usize {
    let op = operations.pop().expect("shouldn't run out here");
    let mut ret = match op {
        "+" => 0,
        "*" => 1,
        _ => panic!("panic")
    };
    let mut cur_col = col;
    while numbers.iter().any(|row| {
        row[cur_col] != ' '
    }) {
        let mut term = 0;

        let mut mult = 1;
        for row in (0..numbers.len()).rev() {
            let digit = match numbers[row][cur_col] {
                ' ' => 0,
                _ => numbers[row][cur_col].to_digit(10).expect("should be a digit") as usize
            };
            term += digit * mult;

            mult *= 10;
        }
        while term > 0 && term % 10 == 0 {
            term /= 10;
        }
        
        // println!("\t{}", format!("{term} {op} {ret}"));
        match op {
            "+" => ret += term,
            "*" => ret *= term,
            _ => panic!("panic")
        };
        if cur_col == 0 {
            break;
        }
        cur_col -= 1;
    }

    // println!("Res: {ret}");
    ret
}

fn parse(data: &str) -> (Vec<Vec<char>>, Vec<&str>) {
    let mut lines = data.split("\n").collect_vec();
    let operations = lines.pop().unwrap().split_whitespace().collect_vec();
    let numbers = lines.iter().map(|line| {
        line.chars().collect_vec()
    }).collect_vec();
    (numbers, operations)
}
