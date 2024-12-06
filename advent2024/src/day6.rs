use std::collections::{HashMap,HashSet};
enum Tile {
    Start,
    Empty,
    Wall
}
const DIRECTIONS: [Point; 4] = [(0,-1), (1,0), (0,1), (-1,0)];

type Point = (isize,isize);
type Grid = HashMap<Point, Tile>;

fn parse (data: &str) -> (Grid, Point) {
    let mut grid: Grid = HashMap::new();
    let mut start: Point = (0,0);
    data.split("\n").enumerate().for_each(|(y,line)| {
        line.chars().enumerate().for_each(|(x,char)| {
            let tile = match char {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                '^' => Tile::Start,
                _ => Tile::Empty
            };

            if matches!(tile, Tile::Start) {
                start = (x as isize,y as isize);
            }
            grid.insert((x as isize,y as isize), tile);
        });
    });
    (grid,start)
}

fn walk(cur: Point, dir: Point) -> Point {
    (cur.0 + dir.0, cur.1 + dir.1)
}

fn will_loop(grid: &Grid, start: Point) -> bool {
    let mut mem: HashSet<(Point, usize)> = HashSet::new();
    let mut cur = start;
    let mut dir = 0;
    while grid.contains_key(&cur) {
        if mem.contains(&(cur,dir)) {
            return true;
        }
        let tile = grid.get(&cur).unwrap();
        match tile {
            Tile::Empty | Tile::Start => {
                mem.insert((cur,dir));
            },
            Tile::Wall => {
                cur = walk(cur, DIRECTIONS[(dir + 2) % 4]);
                dir += 1;
                dir %= 4;
            }
        }
        let vector = DIRECTIONS[dir];
        cur = walk(cur, vector);
    }
    false
}

fn part1 (grid: &Grid, start: Point) -> HashSet<Point> {
    let mut mem = HashSet::<Point>::new();
    mem.insert(start);

    let mut cur = start;
    let mut dir = 0;

    while grid.contains_key(&cur) {
        let tile = grid.get(&cur).unwrap();
        match tile {
            Tile::Empty | Tile::Start => {
                mem.insert(cur);
            },
            Tile::Wall => {
                cur = walk(cur, DIRECTIONS[(dir + 2) % 4]);
                dir += 1;
                dir %= 4;
            }
        }
        let vector = DIRECTIONS[dir];
        cur = walk(cur, vector);
    }
    mem
}
pub async fn advent_1(data: String) -> usize {
    let (grid,start) = parse(&data);
    let mem = part1(&grid, start);
    mem.len()
}

pub async fn advent_2(data: String) -> usize {
    let mut ret = 0;
    let (mut grid,start) = parse(&data);
    let mem = part1(&grid, start);
    for location in mem {
        if location == start {
            continue;
        }

        grid.insert(location, Tile::Wall);
        if will_loop(&grid, start) {
            ret += 1;
        }
        grid.insert(location, Tile::Empty);
    }
    ret
}

fn _print (grid: &Grid, mem: &HashSet<Point>) {
    let mut x = 0;
    let mut y = 0;
    while grid.contains_key(&(x,y)) {
        let mut line = vec![];
        while grid.contains_key(&(x,y)) {
            let mut c = match grid.get(&(x,y)).unwrap() {
                Tile::Empty => '.',
                Tile::Wall => '#',
                Tile::Start => '^'
            };
            if mem.contains(&(x,y)) {
                c = 'X';
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
