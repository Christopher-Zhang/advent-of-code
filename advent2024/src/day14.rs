use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;
use crate::util::*;

struct Robot {
    vel: Point,
    pos: Point
}

const ROOM_HEIGHT: isize = 103;
const ROOM_WIDTH: isize = 101;
// const ROOM_HEIGHT: isize = 7;
// const ROOM_WIDTH: isize = 11;
pub async fn advent_1(data: String) -> usize {
    let mut robots = parse(&data);
    simulate(&mut robots, 100);

    get_safety_factor(&robots)
}

fn parse(data: &str) -> Vec<Robot> {
    data.split("\n").map(|line| {
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let caps = re.captures(line).unwrap().iter().skip(1).map(|cap| cap.unwrap().as_str().parse::<isize>().unwrap()).collect_vec();
        Robot{
            pos: (caps[0], caps[1]),
            vel: (caps[2], caps[3])
        }
    }).collect_vec()
}

fn simulate(robots: &mut Vec<Robot>, seconds: isize) {
    robots.iter_mut().for_each(|robot| {
        robot.pos.0 += (robot.vel.0 + ROOM_WIDTH) * seconds;
        robot.pos.1 += (robot.vel.1 + ROOM_HEIGHT) * seconds;
        robot.pos.0 %= ROOM_WIDTH;
        robot.pos.1 %= ROOM_HEIGHT;
    });
}

fn get_safety_factor(robots: &Vec<Robot>) -> usize {
    let mut quadrants = [0;4];
    robots.iter().for_each(|robot| {
        let x = robot.pos.0;
        let y = robot.pos.1;
        let x_mid = ROOM_WIDTH / 2;
        let y_mid = ROOM_HEIGHT / 2;
        if x < x_mid && y < y_mid {
            quadrants[0] += 1;
        }
        else if x > x_mid && y < y_mid {
            quadrants[1] += 1;
        }
        else if x < x_mid && y > y_mid {
            quadrants[2] += 1;
        }
        else if x > x_mid && y > y_mid {
            quadrants[3] += 1;
        }
    });


    quadrants.iter().fold(1, |x,y| x * y)
}

pub async fn advent_2(data: String) -> usize {
    let mut robots = parse(&data);
    // let mut seen = HashSet::new();
    simulate(&mut robots, 7300);
    for i in 0..10000 {
        simulate(&mut robots, 1);
        println!("iteration {}", i+7300);
        // let robot_string = stringify_robots(&robots);
        // if seen.contains(&robot_string) {
        //     println!("FOUND TREE?");
        //     println!("iter : {i}");
        //     print_robots(&robots);
        //     return 0;
        // }
        // seen.insert(robot_string);
        // if print_robots(&robots) {
        //     println!("iteration {}", i+1);
        // }
        print_robots(&robots);
    }

    0
}

fn stringify_robots(robots: &Vec<Robot>) -> String {
    let mut set = HashSet::<Point>::new();
    robots.iter().for_each(|robot| {
        set.insert(robot.pos);
    });
    let mut chars = Vec::new();
    for y in 0..ROOM_HEIGHT {
        for x in 0..ROOM_WIDTH {
            if set.contains(&(x,y)) {
                // print!("X");
                chars.push('X');
            }
            else {
                // print!(" ");
                chars.push(' ');
            }
        }
        // println!();
    }
    return chars.iter().join("");

}
fn print_robots(robots: &Vec<Robot>) {
    let mut set = HashSet::<Point>::new();
    robots.iter().for_each(|robot| {
        set.insert(robot.pos);
    });
    if (0..ROOM_HEIGHT).any(|y| {
        robots.iter().filter_map(|robot| {
            if robot.pos.1 == y {
                return Some(1)
            }
            None
        }).sum::<i32>() > 20
    }) {
        for y in 0..ROOM_HEIGHT {
            for x in 0..ROOM_WIDTH {
                if set.contains(&(x,y)) {
                    print!("X");
                }
                else {
                    print!(" ");
                }
            }
            println!();
        }
    }

}