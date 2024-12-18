use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use crate::util::*;


pub async fn advent_1(data: String, test: bool) -> usize {
    let (width, height) = match test {
        true => (7,7),
        false => (71,71)
    };
    let bytes = match test {
        true => 12,
        false => 1024
    };
    let mut grid = CGrid::new();
    let corruptions = data.split("\n").map(|line| {
        line.split(",").map(|val| val.parse::<isize>().unwrap()).collect_tuple::<Point>().unwrap()
    }).collect_vec();
    for y in 0..height {
        for x in 0..width {
            grid.insert((x,y), '.');
        }
    }
    for i in 0..bytes {
        if i >= corruptions.len() {
            break;
        } 
        let (x,y) = corruptions[i];
        grid.insert((x,y), '#');
    }
    print_grid(&grid, None);
    djikstra((0,0), (width-1, height-1), &grid)
}
fn djikstra (start: Point, target: Point, grid: &CGrid) -> usize {
    let mut seen = HashMap::<Point,usize>::new();
    let mut q = VecDeque::<(Point,usize)>::new();
    q.push_back((start, 0));
    while !q.is_empty() {
        let (cur_point, cur_steps) = q.pop_front().unwrap();
        if let Some(steps) = seen.get(&cur_point) {
            if cur_steps >= *steps {
                continue;
            }
        }
        if cur_point == target {
            return cur_steps;
        }
        seen.insert(cur_point, cur_steps);
        for dir in CARDINAL_DIRECTIONS {
            let next = walk_grid(cur_point, dir);
            if !seen.contains_key(&next) {
                if let Some(next_tile) = grid.get(&next) {
                    if *next_tile == '.' {
                        q.push_back((next, cur_steps + 1));
                    }
                }
            }
        }
    }
    return 0;
}
pub async fn advent_2(data: String, test: bool) -> Point {
    let (width, height) = match test {
        true => (7,7),
        false => (71,71)
    };
    let bytes = match test {
        true => 12,
        false => 1024
    };
    let mut grid = CGrid::new();
    let corruptions = data.split("\n").map(|line| {
        line.split(",").map(|val| val.parse::<isize>().unwrap()).collect_tuple::<Point>().unwrap()
    }).collect_vec();
    for y in 0..height {
        for x in 0..width {
            grid.insert((x,y), '.');
        }
    }
    for i in 0..usize::MAX {
        if i >= corruptions.len() {
            break;
        } 
        let (x,y) = corruptions[i];
        grid.insert((x,y), '#');
        if i > 1024 {
            if djikstra((0,0), (width-1, height-1), &grid) == 0 {
                return corruptions[i];
            }
        }
    }
    (0,0)
}
