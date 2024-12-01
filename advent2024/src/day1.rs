use std::{collections::HashMap, iter::zip};
use counter::Counter;
use itertools::{Itertools};

// use 
pub async fn advent_1(data: String) -> usize {
    let mut ret = 0;
    let (mut col1, mut col2) = parse(&data);
    col1.sort();
    col2.sort();
    
    zip(col1,col2).for_each(|(a,b)| {
        if a > b {
            ret += a - b;
        }
        else {
            ret += b - a;
        }
    });
    ret
}

pub async fn advent_2(data: String) -> usize {
    let mut ret = 0;
    let (col1, col2) = parse(&data);

    let counts = col2.iter().collect::<Counter<_>>();
    for x in col1 {
        ret += counts.get(&x).unwrap_or(&0) * x;
    }
    ret
}

fn parse(data: &str) -> (Vec<usize>, Vec<usize>) {
    let mut col1: Vec<usize> = vec![];
    let mut col2: Vec<usize> = vec![];

    data.split("\n").for_each(|line| {
        let pair = line.split_whitespace().collect_vec();
        col1.push(pair.get(0).unwrap().parse::<usize>().unwrap());
        col2.push(pair.get(1).unwrap().parse::<usize>().unwrap());
    });

    (col1, col2)
}

fn count(col: &Vec<usize>) -> HashMap<usize, usize> {
    let mut ret = HashMap::new();

    ret
}