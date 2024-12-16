use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use regex::Regex;
use crate::util::*;


pub async fn advent_1(data: String) -> usize {
    let mut ret = 0;
    let prizes = parse(&data);
    for (ax,ay,bx,by,x,y) in prizes {
        ret += djikstra((ax,ay), (bx,by), (x,y));
    }
    ret
}
fn parse (data: &str) -> Vec<(isize, isize, isize, isize, isize, isize)> {
    data.split("\n\n").map(|x| {
        let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();
        let caps = re.captures(x).unwrap().iter().skip(1).map(|cap| {
            cap.unwrap().as_str().parse::<isize>().unwrap()
        }).collect_vec();
        (caps[0], caps[1], caps[2], caps[3], caps[4], caps[5])
    }).collect_vec()
}
fn djikstra (a: Point, b: Point, target: Point) -> usize {
    let mut seen = HashMap::<Point,usize>::new();
    let mut q = VecDeque::<(Point,usize)>::new();
    q.push_back(((0,0), 0));
    while !q.is_empty() {
        let (cur_point, cur_steps) = q.pop_front().unwrap();
        if let Some(steps) = seen.get(&cur_point) {
            if cur_steps >= *steps {
                continue;
            }
        }
        seen.insert(cur_point, cur_steps);
        let next_a = walk_grid(cur_point, a);
        let next_b = walk_grid(cur_point, b);
        if next_a == target {
            return cur_steps + 3;
        }
        if next_b == target {
            return cur_steps + 1;
        }
        if next_a.0 < target.0 && next_a.1 < target.1 {
            q.push_back((next_a, cur_steps + 3));
        }
        if next_b.0 < target.0 && next_b.1 < target.1 {
            q.push_back((next_b, cur_steps + 1));
        }
    }
    return 0;
}
pub async fn advent_2(data: String) -> i64 {
    let mut ret = 0;
    let prizes = parse(&data);
    for (ax,ay,bx,by,x,y) in prizes {
        ret += math(ax as i64,ay as i64,bx as i64,by as i64,(x + 10000000000000) as i64,(y + 10000000000000) as i64);
    }
    ret
}
fn math(ax: i64, ay: i64, bx: i64, by: i64, x: i64, y: i64) -> i64 {
    let bx_new = bx * ay;
    let by_new = by * ax;
    let x_new = x * ay;
    let y_new = y * ax;

    let left = bx_new - by_new;
    let right = x_new - y_new;
    let one = right / left;
    let three = (x - bx * one) / ax;
    if ax * three + bx * one == x && ay * three + by * one == y {
        return one + three * 3;
    }
    0
}
