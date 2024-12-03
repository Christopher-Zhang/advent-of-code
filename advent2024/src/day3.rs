use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;


pub async fn advent_1(data: String) -> usize {
    let mut ret = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for (_,[a,b]) in re.captures_iter(&data).map(|c| c.extract()) {
        ret += a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap();
    }
    ret
}

pub async fn advent_2(data: String) -> usize {
    let mut ret = 0;
    let re = Regex::new(r"(don't)\(\)()|(do)\(\)()|mul\((\d+),(\d+)\)").unwrap();
    let mut allow = true;
    for (_,[a,b]) in re.captures_iter(&data).map(|c| c.extract()) {
        if a == "do" {
            allow = true;
        }
        else if a == "don't" {
            allow = false;
        }
        else if allow {
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            ret += a * b;
        }
    }
    ret
}
