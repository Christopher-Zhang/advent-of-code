use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

use crate::util::*;


pub async fn advent_1(data: String) -> usize {
    let grid = parse_digit_grid(&data);
    let starts: Vec<Point> = grid.keys().filter_map(|pt| {
        let height = match grid.get(pt) {
            Some(v) => *v,
            None => 1
        };
        if height == 0 {
            return Some(*pt);
        }
        None
    }).collect_vec();
    starts.iter().map(|start| count_trailheads(&grid, *start)).sum()
}

pub async fn advent_2(data: String) -> usize {
    let grid = parse_digit_grid(&data);
    let starts: Vec<Point> = grid.keys().filter_map(|pt| {
        let height = match grid.get(pt) {
            Some(v) => *v,
            None => 1
        };
        if height == 0 {
            return Some(*pt);
        }
        None
    }).collect_vec();
    let mut seen = HashSet::new();
    starts.iter().map(|point| count_trails(&grid, *point, &mut seen)).sum()
}

fn count_trailheads(grid: &UGrid, cur: Point) -> usize {
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(cur);
    seen.insert(cur);
    while !q.is_empty() {
        let cur = q.pop_front().unwrap();
        let height = grid.get(&cur).unwrap();
        CARDINAL_DIRECTIONS.iter().for_each(|dir| {
            let next = walk_grid(cur, *dir);
            if let Some(next_height) = grid.get(&next) {
                if (height < next_height && next_height - height == 1) && !seen.contains(&next) {
                    q.push_back(next);
                    seen.insert(next);
                }
            }
        })
    }
    seen.iter().map(|point| {
        if grid.get(point).unwrap_or(&0) == &9 {
            return 1;
        }
        return 0;
    }).sum()
}
fn count_trails(grid: &UGrid, cur: Point, seen: &mut HashSet<Point>) -> usize {
    if let Some(height) = grid.get(&cur) {
        if *height == 9 {
            return 1;
        }
        seen.insert(cur);
        let ret = CARDINAL_DIRECTIONS.iter().map(|dir| {
            let next = walk_grid(cur, *dir);
            if seen.contains(&next) {
                return 0;
            }
            if let Some(next_height) = grid.get(&next) {
                if next_height > height && next_height - height == 1 {
                    return count_trails(grid, next, seen);
                }
            }
            return 0;
        }).sum();
        seen.remove(&cur);
        return ret;
    }
    0
}
