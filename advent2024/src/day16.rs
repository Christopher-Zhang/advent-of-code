use std::collections::{HashMap, HashSet, VecDeque};
use crate::util::*;


pub async fn advent_1(data: String) -> usize {
    let grid = parse_cgrid(&data);
    let start = grid.iter().filter_map(|(point, c)| {
        if *c == 'S' {
            return Some(*point);
        }
        None
    }).fold((0,0), |_,b| b);
    let target = grid.iter().filter_map(|(point, c)| {
        if *c == 'E' {
            return Some(*point);
        }
        None
    }).fold((0,0), |_,b| b);

    djikstra(start, 1, target, &grid)
}
fn djikstra (start_pos: Point, start_dir_index: usize, target: Point, grid: &CGrid) -> usize {
    let mut seen = HashMap::<(Point, usize),usize>::new();
    let mut q = VecDeque::<(Point, usize ,usize)>::new();
    q.push_back((start_pos, start_dir_index, 0));
    let mut best = usize::MAX;
    while !q.is_empty() {
        let (cur_pos, cur_dir_index, cur_score) = q.pop_front().unwrap();
        if let Some(score) = seen.get(&(cur_pos, cur_dir_index)) {
            if cur_score >= *score {
                continue;
            }
        }
        seen.insert((cur_pos, cur_dir_index), cur_score);
        if cur_pos == target {
            best = std::cmp::min(best, cur_score);
            continue;
        }
        let cur_dir = CARDINAL_DIRECTIONS[cur_dir_index];
        let next_pos = walk_grid(cur_pos, cur_dir);
        if let Some(next_tile) = grid.get(&next_pos) {
            if *next_tile != '#' {
                q.push_back((next_pos, cur_dir_index, cur_score + 1));
            }
        }
        for i in 1..(CARDINAL_DIRECTIONS.len()) {
            let new_dir = (cur_dir_index + i) % CARDINAL_DIRECTIONS.len();
            q.push_back((cur_pos, new_dir, cur_score + 1000));
        }
    }
    best
}

pub async fn advent_2(data: String) -> usize {
    let grid = parse_cgrid(&data);
    let start = grid.iter().filter_map(|(point, c)| {
        if *c == 'S' {
            return Some(*point);
        }
        None
    }).fold((0,0), |_,b| b);
    let target = grid.iter().filter_map(|(point, c)| {
        if *c == 'E' {
            return Some(*point);
        }
        None
    }).fold((0,0), |_,b| b);

    let best = djikstra(start, 1, target, &grid);
    let points = djikstra_with_tracking(start, 1, target, best, &grid);
    print_grid(&grid, Some(&points));

    points.len()
}

fn djikstra_with_tracking (start_pos: Point, start_dir_index: usize, target: Point, best: usize, grid: &CGrid) -> HashSet<Point> {
    let mut seen = HashMap::<(Point, usize),usize>::new();
    let mut q = VecDeque::<(Point, usize ,usize, Vec<Point>)>::new();
    q.push_back((start_pos, start_dir_index, 0, vec![]));
    let mut ret = HashSet::new();
    ret.insert(start_pos);
    while !q.is_empty() {
        let (cur_pos, cur_dir_index, cur_score, mut path) = q.pop_front().unwrap();
        if cur_score > best {
            continue;
        }
        if let Some(score) = seen.get(&(cur_pos, cur_dir_index)) {
            if cur_score > *score {
                continue;
            }
        }
        seen.insert((cur_pos, cur_dir_index), cur_score);
        if cur_pos == target && cur_score == best {
            path.iter().for_each(|p| {
                ret.insert(*p);
            });
            continue;
        }
        let cur_dir = CARDINAL_DIRECTIONS[cur_dir_index];
        let next_pos = walk_grid(cur_pos, cur_dir);
        if cur_score + 1000 < best {
            for i in 1..(CARDINAL_DIRECTIONS.len()) {
                let new_dir = (cur_dir_index + i) % CARDINAL_DIRECTIONS.len();
                q.push_back((cur_pos, new_dir, cur_score + 1000, path.clone()));
            }
        }
        if let Some(next_tile) = grid.get(&next_pos) {
            if *next_tile != '#' {
                path.push(next_pos);
                q.push_back((next_pos, cur_dir_index, cur_score + 1, path));
            }
        }
    }
    ret
}