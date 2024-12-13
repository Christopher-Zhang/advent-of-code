use std::collections::{HashMap, HashSet};
use itertools::Itertools;

use crate::util::*;

pub async fn advent_1(data: String) -> usize {
    let mut ret = 0;
    let grid = parse_cgrid(&data);
    let mut inserted: HashSet<Point> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    while grid.contains_key(&(x,y)) {
        while grid.contains_key(&(x,y)) {
            if !inserted.contains(&(x,y)) {
                let c = grid.get(&(x,y)).unwrap();
                let (area, perimeter) = traverse(&grid, &mut inserted, *c, (x,y));
                ret += area * perimeter;
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    ret
}
fn traverse(grid: &CGrid, inserted: &mut HashSet<Point>, plant: char, cur: Point) -> (usize, usize) {
    let mut area = 0;
    let mut perimeter = 0;
    if let Some(cur_plant) = grid.get(&cur) {
        if *cur_plant != plant || inserted.contains(&cur) {
            return (area,perimeter);
        }
        inserted.insert(cur);
        area += 1;
        for dir in CARDINAL_DIRECTIONS {
            let next = walk_grid(cur, dir);
            if let Some(next_plant) = grid.get(&next) {
                if *next_plant == plant {
                    let (a,b) = traverse(grid, inserted, plant, next);
                    area += a;
                    perimeter += b;
                }
                else {
                    perimeter += 1;
                }
            }
            else {
                perimeter += 1;
            }
        }
        
    }
    return (area,perimeter);
}

pub async fn advent_2(data: String) -> usize {
    let mut ret = 0;
    let grid = parse_cgrid(&data);
    let mut inserted: HashSet<Point> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    while grid.contains_key(&(x,y)) {
        while grid.contains_key(&(x,y)) {
            if !inserted.contains(&(x,y)) {
                let c = grid.get(&(x,y)).unwrap();
                let mut perimeter = HashSet::new();
                let area = traverse2(&grid, &mut inserted, &mut perimeter, *c, (x,y));
                let sides = get_sides(&grid, &mut perimeter, *c);
                ret += area * sides;
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    ret
}

fn get_sides(grid: &CGrid, perimeter: &mut HashSet<Point>, c: char) -> usize {
    let mut ret = 0;
    let mut seen: HashSet<(Point, Point)> = HashSet::new();
    for point in perimeter.iter() {
        for i in 0..CARDINAL_DIRECTIONS.len() {
            let dir = CARDINAL_DIRECTIONS[i];
            let mut cur = *point;
            if seen.contains(&(cur, dir)) {
                continue;
            }
            if grid.get(&walk_grid(cur, dir)).is_none_or(|x| *x != c) {
                let new_i = (i+1) % CARDINAL_DIRECTIONS.len();
                while perimeter.contains(&cur) && grid.get(&walk_grid(cur,dir)).is_none_or(|x| *x != c) {
                    seen.insert((cur,dir));
                    let new_dir = CARDINAL_DIRECTIONS[new_i];
                    cur = walk_grid(cur,new_dir);
                }
                cur = *point;
                let new_i = (i+3) % CARDINAL_DIRECTIONS.len();
                while perimeter.contains(&cur) && grid.get(&walk_grid(cur,dir)).is_none_or(|x| *x != c) {
                    seen.insert((cur,dir));
                    let new_dir = CARDINAL_DIRECTIONS[new_i];
                    cur = walk_grid(cur,new_dir);
                }
                ret += 1;
            }
            
        }
    }

    ret
}
fn traverse2(grid: &CGrid, inserted: &mut HashSet<Point>, perimeter: &mut HashSet<Point>, plant: char, cur: Point) -> usize {
    let mut area = 0;
    if let Some(cur_plant) = grid.get(&cur) {
        if *cur_plant != plant || inserted.contains(&cur) {
            return area
        }
        inserted.insert(cur);
        area += 1;
        for dir in CARDINAL_DIRECTIONS {
            let next = walk_grid(cur, dir);
            if let Some(next_plant) = grid.get(&next) {
                if *next_plant == plant {
                    area += traverse2(grid, inserted, perimeter, plant, next);
                }
                else {
                    perimeter.insert(cur);
                }
            }
            else {
                perimeter.insert(cur);
            }
        }
        
    }
    area
}

