use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use crate::util::*;


pub async fn advent_1(data: String, _test: bool) -> usize {
    let mut ret = 0;
    let (mut grid, start, end) = parse_maze(&data, 'S', 'E');
    println!("start {start:?} end {end:?}");
    print_grid(&grid, None);
    let best_time = djikstra(start, end, &grid);
    println!("Best non-cheating time is {best_time}");
    ret
}

fn solve(grid: &CGrid, start: Point, end: Point, best_time: usize) -> HashMap<(Point, Point), usize> {
    let ret = HashMap::new();
    let mut seen = HashMap::<Point,usize>::new();
    let mut q: VecDeque<(Point, Option<(Point,Point)>, usize)> = VecDeque::new();
    q.push_back((start, None, 0));
    while !q.is_empty() {
        let (cur_point, cur_cheat, cur_steps) = q.pop_front().unwrap();
        if let Some(steps) = seen.get(&cur_point) {
            if cur_steps >= *steps {
                continue;
            }
        }
        if cur_point == end && cur_cheat.is_some(){
            if let Some((cheat)) = cur_cheat {
                if let Some(best) = ret.get(&cheat) {
                    ret.insert(cheat, std::cmp::min(*best, cur_steps));
                }
            }
        }
        seen.insert(cur_point, cur_steps);
        for dir in CARDINAL_DIRECTIONS {
            let next = walk_grid(cur_point, dir);
            if !seen.contains_key(&next) {
                if let Some(next_tile) = grid.get(&next) {
                    if *next_tile == '.' {
                        q.push_back((next, cur_cheat, cur_steps + 1));
                    }
                    else if *next_tile == '#' {
                        let cheated_point = walk_grid(next, dir);
                        if cur_cheat.is_none() && grid.get(&walk_grid(cur, dir)
                    }
                }
            }
        }
    }

    ret
}
pub async fn advent_2(data: String, _test: bool) -> usize {
    let mut ret = 0;

    ret
}
