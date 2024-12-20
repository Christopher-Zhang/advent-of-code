use std::collections::{HashMap, HashSet, VecDeque};
use counter::Counter;
use itertools::Itertools;
use crate::util::*;


pub async fn advent_1(data: String, _test: bool) -> usize {
    let (mut grid, start, end) = parse_maze(&data, 'S', 'E');
    println!("start {start:?} end {end:?}");
    print_grid(&grid, None);
    let best_time = djikstra(start, end, &grid);
    println!("Best non-cheating time is {best_time}");
    let threshold = match _test {
        true => 0,
        false => 100
    };

    let cache = get_cache(start, end, &grid);
    solve_optimized(&grid, start, cache, 3, best_time, threshold)
    // let cheats = solve(&grid, start, end, best_time, threshold);
    // let counts = cheats.iter().map(|(_,a)| *a);
    // .collect::<Counter<_>>();
    // dbg!(counts.most_common_ordered());
    // cheats.iter().map(|(_cheat, time)| {
    //     // println!("For cheat {_cheat:?} time save is {}", best_time - time);
    //     if best_time - time >= threshold {
    //         return 1;
    //     }
    //     0
    // }).sum()
}
// #[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
// struct Cheat {
//     a: Point,
//     b: Point,
//     used: bool
// }
fn get_cache(start: Point, target: Point, grid: &CGrid) -> HashMap::<Point, usize> {
    let mut seen = HashMap::<Point,usize>::new();
    let mut q = VecDeque::new();
    q.push_back((target, 0));
    while !q.is_empty() {
        let (cur_point, cur_steps) = q.pop_front().unwrap();
        if let Some(steps) = seen.get(&cur_point) {
            if cur_steps >= *steps {
                continue;
            }
        }
        seen.insert(cur_point, cur_steps);
        for dir in CARDINAL_DIRECTIONS {
            let next = walk_grid(cur_point, dir);
            if !seen.contains_key(&next) {
                if let Some(next_tile) = grid.get(&next) {
                    if *next_tile != '#' {
                        q.push_back((next, cur_steps + 1));
                    }
                }
            }
        }
    }
    seen
}

fn solve_optimized(grid: &CGrid, start: Point, cache: HashMap::<Point, usize>, cheat_dist: usize, best_time: usize, threshold: usize) -> usize {
    let mut cheats = HashMap::new();
    // let mut seen = HashSet::new();
    let mut ret = 0;
    let base_time = *cache.get(&start).expect("didn't find start?");
    let cheat_dist = cheat_dist as isize;
    for (cur, start_dist) in cache.iter() {
        for dir in CARDINAL_DIRECTIONS {
            let start_cheat = walk_grid(*cur, dir);
            if let Some(tile) = grid.get(&start_cheat) {
                if *tile != '#' {
                    continue;
                }
                for y in -cheat_dist..cheat_dist {
                    for x in -cheat_dist..cheat_dist {
                        let cheat_steps = num::abs(x) + num::abs(y);
                        if cheat_steps >= cheat_dist  {
                            continue;
                        }
                        let end_point = walk_grid(*cur, (x,y));
                        if let Some(end_tile) = grid.get(&end_point) {
                            if *end_tile != '#' {
                                if let Some(remaining) = cache.get(&end_point) {
                                    let new_time = (base_time - start_dist) + remaining + (cheat_steps as usize);
                                    if new_time + threshold <= base_time {
                                        let saved = base_time - new_time;
                                        if !cheats.contains_key(&(cur, end_point)) {
                                            cheats.insert((cur, end_point), saved);
                                            ret += 1;
                                        }
                                    }
                                }
                            }
                        }
        
                    }
                }
            }
        }
    }
    // dbg!(cheats);
    let counts = cheats.iter().map(|(_,b)| b).collect::<Counter<_>>();
    // dbg!(counts);
    ret
}
type Cheat = (Point, Point);
fn solve(grid: &CGrid, start: Point, end: Point, best_time: usize, threshold: usize) -> HashMap<Cheat, usize> {
    let mut ret = HashMap::new();
    let mut seen = HashMap::<(Point, Option<Cheat>),usize>::new();
    let mut q: VecDeque<(Point, Option<Cheat>, usize)> = VecDeque::new();
    q.push_back((start, None, 0));
    while !q.is_empty() {
        let (cur_point, cur_cheat, cur_steps) = q.pop_front().unwrap();
        if cur_steps > best_time {
            continue;
        }
        if let Some(steps) = seen.get(&(cur_point, None)) {
            if cur_steps >= *steps || cur_steps > best_time {
                continue;
            }
        }
        if let Some(steps) = seen.get(&(cur_point, cur_cheat)) {
            if cur_steps >= *steps || cur_steps > best_time {
                continue;
            }
        }
        if cur_point == end {
            if let Some(cheat) = cur_cheat {
                if let Some(best_so_far) = ret.get(&cheat) {
                    ret.insert(cheat, std::cmp::min(*best_so_far, cur_steps));
                }
                else {
                    ret.insert(cheat, cur_steps);
                }
            }
            continue;
        }
        seen.insert((cur_point, cur_cheat), cur_steps);
        for dir in CARDINAL_DIRECTIONS {
            let next = walk_grid(cur_point, dir);
            if let Some(next_tile) = grid.get(&next) {
                if *next_tile != '#' {
                    q.push_back((next, cur_cheat, cur_steps + 1));
                }
                else {
                    let cheated_point = walk_grid(next, dir);
                    if cur_cheat.is_none() && grid.get(&cheated_point).is_some_and(|x| *x != '#') {
                        let new_cheat = (next, cheated_point);
                        if let Some(steps) = seen.get(&(cheated_point, Some(new_cheat))) {
                            if cur_steps >= *steps || cur_steps > best_time || best_time - cur_steps < threshold {
                                continue;
                            }
                        }
                        if !ret.contains_key(&new_cheat) {
                            q.push_back((cheated_point, Some(new_cheat), cur_steps + 2));
                        }
                    }
                }
            }
        }
    }
    ret
}
pub async fn advent_2(data: String, _test: bool) -> usize {
    let (grid, start, end) = parse_maze(&data, 'S', 'E');
    let threshold = match _test {
        true => 50,
        false => 100
    };

    let cache = get_cache(start, end, &grid);
    solve_optimized(&grid, start, cache, 21, 0, threshold)
}
