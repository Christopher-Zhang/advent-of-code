use std::{cmp::max, collections::HashMap, collections::HashSet};
use crate::util::*;
use gcd::Gcd;


pub async fn advent_1(data: String) -> usize {
    let (nodes, x_max, y_max) = parse(&data);
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (_, points) in nodes.iter() {
        for i in 0..points.len() {
            for j in i+1..points.len() {
                let a = points[i];
                let b = points[j];
                let dist = point_dist(a,b);
                let anti1 = walk_grid(a, dist);
                let anti2 = walk_grid(b, (-dist.0, -dist.1));
                if in_bounds(anti1, x_max, y_max) {
                    antinodes.insert(anti1);
                }
                if in_bounds(anti2, x_max, y_max) {
                    antinodes.insert(anti2);
                }
            }
        }
    }
    antinodes.len()
}

pub async fn advent_2(data: String) -> usize {
    let (nodes, x_max, y_max) = parse(&data);
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (_, points) in nodes.iter() {
        for i in 0..points.len() {
            for j in i+1..points.len() {
                let a = points[i];
                let b = points[j];
                let dist = point_dist(a,b);
                let div = (dist.0.abs() as usize).gcd(dist.1.abs() as usize) as isize;
                let dist = (dist.0/div, dist.1/div);
                let mut anti1 = a;
                let mut anti2 = b;
                loop {
                    anti1 = walk_grid(anti1, dist);
                    if !in_bounds(anti1, x_max, y_max) {
                        break;
                    }
                    antinodes.insert(anti1);
                }
                loop {
                    anti2 = walk_grid(anti2, (-dist.0, -dist.1));
                    if !in_bounds(anti2, x_max, y_max) {
                        break;
                    }
                    antinodes.insert(anti2);
                }
                anti1 = a;
                anti2 = b;
                loop {
                    anti2 = walk_grid(anti2, dist);
                    antinodes.insert(anti2);
                    if anti2 == anti1 {
                        break;
                    }
                }
                antinodes.insert(b);
            }
        }
    }
    antinodes.len()
}

fn parse(data: &str) -> (HashMap<char, Vec<Point>>, isize, isize) {
    let mut nodes: HashMap<char, Vec<Point>> = HashMap::new();
    let mut x_max = 0;
    let mut y_max = 0;
    data.split("\n").enumerate().for_each(|(y,line)| {
        y_max = max(y, y_max);
        line.chars().enumerate().for_each(|(x,c)| {
            x_max = max(x, x_max);
            if c == '.' {
                return;
            }
            if nodes.contains_key(&c) {
                nodes.get_mut(&c).unwrap().push((x as isize,y as isize));
            }
            else {
                nodes.insert(c, vec![(x as isize,y as isize)]);
            }
        })
    });
    (nodes, x_max as isize, y_max as isize)
}
