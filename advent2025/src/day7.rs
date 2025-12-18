use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::util::*;


pub async fn advent_1(data: String, _test: bool) -> usize {
    let (mut grid, start, _) = parse_maze(&data, 'S', '.');
    let mut seen = HashSet::new();
    let ret = recur (&mut grid, start, &mut seen);

    print_grid(&grid, None);
    ret
}
fn recur(grid: &mut CGrid, cur: Point, seen: &mut HashSet<Point>) -> usize {
    if seen.contains(&cur) {
        return 0;
    }
    seen.insert(cur);
    if let Some(cur_tile) = grid.get(&cur) {
        return match cur_tile {
            'S'|'.'|'|' => {
                if *cur_tile == '.' {
                    grid.insert(cur, '|');
                }

                return recur(grid, walk_grid(cur, (0,1)), seen);
            },
            '^' => {
                1 + recur(grid, walk_grid(cur, (1,0)), seen) + recur(grid, walk_grid(cur, (-1,0)), seen)
            },
            _ => panic!("bad char")
        };
    }
    0
}

pub async fn advent_2(data: String, _test: bool) -> usize {
    let (mut grid, start, _) = parse_maze(&data, 'S', '.');
    let mut cache = HashMap::new();

    recur2(&mut grid, start, &mut cache)
}

fn recur2(grid: &mut CGrid, cur: Point, cache: &mut HashMap<Point, usize>) -> usize {
    if let Some(cached) = cache.get(&cur) {
        return *cached;
    }

    if let Some(cur_tile) = grid.get(&cur) {
        return match cur_tile {
            'S'|'.'|'|' => {
                if *cur_tile == '.' {
                    grid.insert(cur, '|');
                }

                let val = recur2(grid, walk_grid(cur, (0,1)), cache);
                cache.insert(cur, val);
                val
            },
            '^' => {
                let val = recur2(grid, walk_grid(cur, (1,0)), cache) + recur2(grid, walk_grid(cur, (-1,0)), cache);
                cache.insert(cur, val);
                val
            },
            _ => panic!("bad char")
        };
    }
    1
}
