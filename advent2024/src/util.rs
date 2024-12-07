use std::collections::{HashMap,HashSet};
pub type Point = (isize, isize);
pub type CGrid = HashMap<Point, char>;
pub const CARDINAL_DIRECTIONS: [isize; 4] = [(0,-1), (1,0), (0,1), (-1,0)];
pub const EIGHT_DIRECTIONS: [isize; 8] = [(0,-1), (1,-1), (1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1)];
pub fn walk_grid(cur: Point, dir: Point) -> Point { (cur.0 + dir.0, cur.1 + dir.1) }

pub fn print_grid (grid: &Grid, seen: Option<&HashSet<Point>>) {
    let mut x = 0;
    let mut y = 0;
    while grid.contains_key(&(x,y)) {
        let mut line = vec![];
        while grid.contains_key(&(x,y)) {
            let mut c = grid.get(&(x,y)).unwrap();
            if mem.contains(&(x,y)) {
                c = 'X'
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
