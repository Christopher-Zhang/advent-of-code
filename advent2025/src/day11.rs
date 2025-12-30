use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use bimap::BiMap;
use crate::util::*;


pub async fn advent_1(data: String, _test: bool) -> usize {
    let map = build_map(&data);
    pathfind("you", &map)
}

pub async fn advent_2(data: String, _test: bool) -> usize {
    let map = build_map(&data);
    let map = compress(&map);
    dbg!(&map);
    pathfind("svr", &map)
}

fn build_map (data: &str) -> HashMap<String, Vec<String>> {
    let mut ret = HashMap::new();
    data.split("\n").for_each(|line| {
        let split = line.split(" ").collect_vec();
        let dests = split[1..].iter().map(|x| x.to_string()).collect_vec();
        ret.insert(split[0].trim_end_matches(":").to_string(), dests);
    });
    ret
}

fn pathfind (cur_node: &str, map: &HashMap<String, Vec<String>>) -> usize {
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

fn compress (map: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>>{
    let mut ret = HashMap::new();
    let mut reductions: HashMap<String, String> = HashMap::new();
    // let keys = map.keys().copied().collect_vec();

    map.iter().for_each(|(k,v)| {
        if v.len() == 1 {
            let val = &v[0];
            if !reductions.contains_key(k) {
                if let Some(dest) = reductions.get(val) {
                    ret.insert(k.to_string(),vec![dest.to_string()]);
                }
                else {
                    // ret.insert(k.to_string(),vec![val.to_string()]);
                    let mut next = v;
                    let mut cur = vec![k.to_string()];
                    loop {
                        if next.len() > 1 {
                            break;
                        }

                        let next_val = next[0].clone();
                        if next_val == "dac" || next_val == "out" || next_val == "fft" {
                            break;
                        }

                        next = map.get(&next_val).expect("should exist");
                        cur.push(next_val);
                    }
                    let last = cur.pop().expect("should be one");
                    cur.iter().for_each(|x| {
                        reductions.insert(x.to_string(), last.to_string());
                    });
                    println!("inserting {k} -> [{last}]");
                    ret.insert(k.to_string(), vec![next[0].to_string()]);
                }
            }
        }
        else {
            ret.insert(k.to_string(),v.clone());
        }
    });
    ret
}

