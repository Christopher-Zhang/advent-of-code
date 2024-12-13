use std::collections::{HashMap,HashSet};
use itertools::Itertools;

// Grid
pub type Point = (isize, isize);
pub type CGrid = HashMap<Point, char>;
pub type UGrid = HashMap<Point, usize>;
pub const CARDINAL_DIRECTIONS: [Point; 4] = [(0,-1), (1,0), (0,1), (-1,0)];
pub const EIGHT_DIRECTIONS: [Point; 8] = [(0,-1), (1,-1), (1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1)];
pub const DIAGONAL_DIRECTIONS: [Point; 4] = [(1,-1), (1,1), (-1,1), (-1,-1)];
pub fn walk_grid(cur: Point, dir: Point) -> Point { (cur.0 + dir.0, cur.1 + dir.1) }
pub fn parse_cgrid(data: &str) -> CGrid {
    let mut grid = CGrid::new();
    data.split("\n").enumerate().for_each(|(y,line)| {
        line.chars().enumerate().for_each(|(x,c)| {
            grid.insert((x as isize,y as isize), c);
        })
    });
    grid
}
pub fn parse_digit_grid(data: &str) -> UGrid {
    let mut grid = UGrid::new();
    data.split("\n").enumerate().for_each(|(y,line)| {
        line.chars().enumerate().for_each(|(x,c)| {
            grid.insert((x as isize,y as isize), c.to_digit(10).unwrap() as usize);
        })
    });
    grid
}
pub fn point_dist(a: Point, b: Point) -> Point {
    (a.0-b.0, a.1-b.1)
}
pub fn in_bounds(check: Point, x_max: isize, y_max: isize) -> bool {
    check.0 >= 0 && check.0 <= x_max && check.1 >= 0 && check.1 <= y_max
}
pub fn print_grid (grid: &CGrid, seen: Option<&HashSet<Point>>) {
    let mut x = 0;
    let mut y = 0;
    while grid.contains_key(&(x,y)) {
        let mut line = vec![];
        while grid.contains_key(&(x,y)) {
            let mut c = grid.get(&(x,y)).unwrap();
            match seen {
                Some(mem) => {
                    if mem.contains(&(x,y)) {
                        c = &'X';
                    }
                },
                None => ()
            }

            print!("{c}");
            line.push(c);
            x += 1;
        }
        y += 1;
        x = 0;
        println!("");
    }
}

// numbers
pub fn parse_usize_list (data: &str) -> Vec<Vec<usize>> {
    data.split("\n").map(|line| {
        line.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect_vec()
    }).collect_vec()
}
