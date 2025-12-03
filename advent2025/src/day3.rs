use std::collections::HashMap;
use itertools::Itertools;
use crate::util::*;


pub async fn advent_1(data: String, _test: bool) -> usize {
    let mut total = 0;
    let banks = data.split("\n").map(|line| {
        line.chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect_vec()
    }).collect_vec();
    banks.iter().for_each(|bank| {
        let mut max = 0;
        let mut max_index = 0;
        for i in 0..bank.len()-1 {
            let val = bank.get(i).unwrap();
            if val > &max {
                max = *val;
                max_index = i;
            }
        }
        let mut max2 = 0;
        for i in (max_index + 1)..bank.len() {
            let val = bank.get(i).unwrap();
            if val > &max2 {
                max2 = *val;
            }
        }

        total += max * 10 + max2;

    
    });
    total
}

pub async fn advent_2(data: String, _test: bool) -> usize {
    let iters = 12;
    let mut total = 0;
    let banks = data.split("\n").map(|line| {
        line.chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect_vec()
    }).collect_vec();
    banks.iter().for_each(|bank| {
        let mut maxes: Vec<usize> = vec![];
        let mut max_index = 0;
        for iter in 0..iters {
            maxes.push(0);
            for i in max_index..bank.len() - iters + iter + 1 {
                let val = bank[i];
                if val > maxes[iter] {
                    maxes[iter] = val;
                    max_index = i+1;
                }
            }
        }
        let mut subtotal = 0;
        for (i,max) in maxes.iter().enumerate() {
            // dbg!(max * 10_usize.pow((11-i) as u32));
            let subsubtotal = max * 10_usize.pow((11-i) as u32);
            subtotal += subsubtotal;
        }
        // dbg!(maxes.iter().join(""));
        // println!("{subtotal} for ");
        // dbg!(bank.iter().join(""));
        total += subtotal;

    
    });
    total
}
