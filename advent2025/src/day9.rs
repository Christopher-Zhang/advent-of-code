use std::collections::HashMap;
use itertools::Itertools;
use crate::util::*;


pub async fn advent_1(data: String, _test: bool) -> usize {
    let coords = parse(&data);
    let mut max = 0;
    for i in 0..coords.len()-1 {
        for j in i+1..coords.len() {
            let a = coords[i];
            let b = coords[j];
            max = std::cmp::max(max, ((a.0-b.0).abs() + 1) * ((a.1-b.1).abs() + 1));
        }
    }
    max as usize
}

pub async fn advent_2(data: String, _test: bool) -> usize {
    let mut total = 0;

    total
}

fn parse(data: &str) -> Vec<Point> {
    data.split('\n').map(|line| {
        line.split(',').map(|val| {
            val.parse::<isize>().unwrap()
        }).collect_tuple().unwrap()
    }).collect_vec()
}