use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::util::*;


pub async fn advent_1(data: String, _test: bool) -> usize {
    let map = build_map(&data);
    pathfind("you", &map)
}

pub async fn advent_2(data: String, _test: bool) -> usize {
    let map = build_map(&data);
    pathfind("svr", &map)
}

fn build_map (data: &str) -> HashMap<&str, Vec<&str>> {
    let mut ret = HashMap::new();
    data.split("\n").for_each(|line| {
        let split = line.split(" ").collect_vec();
        let dests = split[1..].iter().map(|x| *x).collect_vec();
        ret.insert(split[0].trim_end_matches(":"), dests);
    });
    ret
}

fn pathfind (cur_node: &str, map: &HashMap<&str, Vec<&str>>) -> usize {
    if cur_node == "out" {
        return 1;
    }

    if let Some(nexts) = map.get(cur_node) {
        return nexts.iter().map(|next| {
            pathfind(next, map)
        }).sum();
    }
    0
}

