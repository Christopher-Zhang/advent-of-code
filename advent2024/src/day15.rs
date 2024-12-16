use std::collections::HashMap;
use itertools::Itertools;
use crate::util::*;


pub async fn advent_1(data: String) -> usize {
    let split = data.split("\n\n").collect_vec();
    let mut grid = parse_cgrid(split[0]);
    let moves = split[1].chars().collect_vec();
    let mut robot = grid.iter().filter_map(|(point, c)| {
        if *c == '@' {
            return Some(*point);
        }
        None
    }).fold((0,0), |_,b| b);
    // print_grid(&grid, None);

    for arrow in moves {
        // println!("move: {arrow}");
        if arrow == '\n' {
            continue;
        }
        robot = move_robot(robot, arrow, &mut grid);
        // print_grid(&grid, None);
    }
    sum_coordinates(&grid)
}

fn move_robot(robot: Point, arrow: char, grid: &mut CGrid) -> Point {
    let dir = match arrow {
        '^' => CARDINAL_DIRECTIONS[0],
        '>' => CARDINAL_DIRECTIONS[1],
        'v' => CARDINAL_DIRECTIONS[2],
        '<' => CARDINAL_DIRECTIONS[3],
        _ => panic!("halp")
    };
    let mut next_pos = walk_grid(robot, dir);
    if let Some(next) = grid.get(&next_pos) {
        let mut next = *next;
        if next == '.' {
            grid.insert(robot, '.');
            grid.insert(next_pos, '@');
            return next_pos;
        }
        else if next == 'O' {
            let mut boxes = 1;
            while next == 'O' {
                next_pos = walk_grid(next_pos, dir);
                next = *grid.get(&next_pos).unwrap_or(&'#');
                boxes += 1;
            }
            if next != '#' {
                let new_robot = walk_grid(robot, dir);
                grid.insert(robot, '.');
                grid.insert(new_robot, '@');
                grid.insert(walk_grid(robot, (dir.0 * boxes, dir.1 * boxes)), 'O');
                return new_robot;
            }
        }
    }
    robot
}
fn sum_coordinates (grid: &CGrid) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut ret = 0;
    while grid.contains_key(&(x,y)) {
        while grid.contains_key(&(x,y)) {
            let c = grid.get(&(x,y)).unwrap();
            if *c == 'O' || *c == '[' {
                ret += 100 * y + x;
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }
    ret as usize
}

pub async fn advent_2(data: String) -> usize {
    let split = data.split("\n\n").collect_vec();
    let mut grid = parse_WIDEgrid(split[0]);
    let moves = split[1].chars().collect_vec();
    let mut robot = grid.iter().filter_map(|(point, c)| {
        if *c == '@' {
            return Some(*point);
        }
        None
    }).fold((0,0), |_,b| b);
    // print_grid(&grid, None);
    for arrow in moves {
        // println!("move: {arrow}");
        if arrow == '\n' {
            continue;
        }
        robot = move_FATrobot(robot, arrow, &mut grid);
        // print_grid(&grid, None);
    }
    print_grid(&grid, None);
    sum_coordinates(&grid)
}
fn move_FATrobot(robot: Point, arrow: char, grid: &mut CGrid) -> Point {
    let dir = match arrow {
        '^' => CARDINAL_DIRECTIONS[0],
        '>' => CARDINAL_DIRECTIONS[1],
        'v' => CARDINAL_DIRECTIONS[2],
        '<' => CARDINAL_DIRECTIONS[3],
        _ => panic!("halp")
    };
    let next_pos = walk_grid(robot, dir);
    if let Some(next) = grid.get(&next_pos) {
        let next = *next;
        if next == '.' {
            grid.insert(robot, '.');
            grid.insert(next_pos, '@');
            return next_pos;
        }
        else if next == '[' || next == ']' {
            if can_move_FATbox(next_pos, dir, grid) {
                move_FATbox(next_pos, dir, grid);
                grid.insert(robot, '.');
                grid.insert(next_pos, '@');
                return walk_grid(robot, dir);
            }
        }
    }
    robot
}
const LEFT_BOX: char = '[';
const RIGHT_BOX: char = ']';

fn can_move_FATbox(box_pos: Point, dir: Point, grid: &mut CGrid) -> bool {
    let box_side = *grid.get(&box_pos).unwrap();
    if box_side == '#' {
        return false;
    }
    if box_side == '.' {
        return true;
    }
    
    if dir == CARDINAL_DIRECTIONS[0] || dir == CARDINAL_DIRECTIONS[2] {
        let a; let b;
        if box_side == LEFT_BOX {
            a = walk_grid(box_pos, dir);
            b = walk_grid(a, CARDINAL_DIRECTIONS[1]);
        }
        else {
            b = walk_grid(box_pos, dir);
            a = walk_grid(b, CARDINAL_DIRECTIONS[3]);
        }

        if can_move_FATbox(a, dir, grid) && can_move_FATbox(b, dir, grid) {
            return true;
        }

    }
    else {
        let next = walk_grid(box_pos, dir);
        if can_move_FATbox(next, dir, grid) {
            return true;
        }
    }
    false
}
fn move_FATbox(box_pos: Point, dir: Point, grid: &mut CGrid) -> bool {
    let box_side = *grid.get(&box_pos).unwrap();
    if box_side == '#' {
        return false;
    }
    if box_side == '.' {
        return true;
    }
    
    if dir == CARDINAL_DIRECTIONS[0] || dir == CARDINAL_DIRECTIONS[2] {
        let a; let b;
        if box_side == LEFT_BOX {
            a = walk_grid(box_pos, dir);
            b = walk_grid(a, CARDINAL_DIRECTIONS[1]);
        }
        else {
            b = walk_grid(box_pos, dir);
            a = walk_grid(b, CARDINAL_DIRECTIONS[3]);
        }

        if move_FATbox(a, dir, grid) && move_FATbox(b, dir, grid) {
            grid.insert(a, '[');
            grid.insert(b, ']');
            grid.insert(box_pos, '.');
            if box_side == LEFT_BOX {
                grid.insert(walk_grid(box_pos, CARDINAL_DIRECTIONS[1]), '.');
            }
            else {
                grid.insert(walk_grid(box_pos, CARDINAL_DIRECTIONS[3]), '.');
            }
            return true;
        }

    }
    else {
        let next = walk_grid(box_pos, dir);
        if move_FATbox(next, dir, grid) {
            grid.insert(next, box_side);
            grid.insert(box_pos, '.');
            return true;
        }
    }
    false
}
fn parse_WIDEgrid(data: &str) -> CGrid {
    let mut grid = CGrid::new();
    data.split("\n").enumerate().for_each(|(y,line)| {
        line.chars().enumerate().for_each(|(x,c)| {
            if c == 'O' {
                grid.insert(((x*2) as isize,(y) as isize), '[');
                grid.insert(((x*2+1) as isize,(y)as isize), ']');
            }
            else if c == '@' {
                grid.insert(((x*2) as isize,(y) as isize), c);
                grid.insert(((x*2+1) as isize,(y)as isize), '.');
            }
            else {
                grid.insert(((x*2) as isize,(y) as isize), c);
                grid.insert(((x*2+1) as isize,(y)as isize), c);
            }
        })
    });
    grid
}
