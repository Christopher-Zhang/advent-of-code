use std::collections::{HashMap, HashSet};
use crate::util::*;


pub async fn advent_1(data: String, _test: bool) -> usize {
    let grid = parse_cgrid(&data);

    let mut accessible = HashSet::<Point>::new();
    for (cur, char) in grid.iter() {
        if *char != '@' {
            continue;
        }
        let num_adj: usize = EIGHT_DIRECTIONS.iter().map(|dir| {
            if let Some(adj) = grid.get(&walk_grid(*cur, *dir)) && *adj == '@' {
                    return 1;
            }
            0
        }).sum();
        if num_adj < 4 {
            accessible.insert(*cur);
        }
    }
    accessible.len()
}

pub async fn advent_2(data: String, _test: bool) -> usize {
    let mut grid = parse_cgrid(&data);
    let mut total = 0;

    let mut adjacents = HashMap::<Point, usize>::new();
    for (cur, char) in grid.iter() {
        if *char != '@' {
            continue;
        }
        let count = EIGHT_DIRECTIONS.iter().map(|dir| {
            if let Some(adj) = grid.get(&walk_grid(*cur, *dir)) && *adj == '@' {
                    return 1;
            }
            0
        }).sum();
        adjacents.insert(*cur, count);
    }
    
    for cur in grid.clone().keys() {
        if let Some(num_adj) = adjacents.get(cur) && *num_adj < 4{
            total += cascade(&mut grid, &mut adjacents, cur);
        }
    }

    total
}

pub fn cascade (grid: &mut CGrid, adjacents: &mut HashMap<Point, usize>, cur: &Point) -> usize {
    grid.insert(*cur, 'x');
    adjacents.remove(cur);
    EIGHT_DIRECTIONS.iter().map(|dir| {
        let next = walk_grid(*cur, *dir);
        if let Some(temp) = adjacents.get_mut(&next) {
            let next_adj = *temp;
            adjacents.insert(next, next_adj - 1);
            if next_adj - 1 < 4 {
                return cascade(grid, adjacents, &next);
            }
        }
        0
    }).sum::<usize>() + 1
}
