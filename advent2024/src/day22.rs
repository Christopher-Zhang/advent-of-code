use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub async fn advent_1(data: String, _test: bool) -> usize {
    let starts = data.split("\n").map(|line| line.parse::<i64>().unwrap()).collect_vec();
    starts.iter().map(|num| {
        let mut cur = *num;
        for _i in 0..2000 {
            cur = step(cur);
        }
        cur
    }).sum::<i64>() as usize
}
fn step(num: i64) -> i64 {
    let prune = 16777216;
    let mut ret = num;
    ret = (ret ^ (ret * 64)) % prune;
    ret = (ret ^ (ret / 32)) % prune;
    ret = ret ^ (ret * 2048);
    ret % prune
}


type FourDiffs = (i64, i64, i64, i64);
pub async fn advent_2(data: String, _test: bool) -> usize {
    let starts = data.split("\n").map(|line| line.parse::<i64>().unwrap()).collect_vec();
    let sequences = starts.iter().map(|start| {
        let mut cur = *start;
        (0..=2001).map(move |_| {
            let temp = cur.clone();
            cur = step(cur);
            temp % 10
        }).collect_vec()
    }).collect_vec();
    let mut all_diff_sequences = HashSet::new();
    let maps = sequences.iter().map(|s| {
        let mut diff_sequences = HashMap::<FourDiffs, i64>::new();
        let mut diffs = (s[1]-s[0], s[2]-s[1], s[3]-s[2], s[4]-s[3]);
        s.iter().skip(4).collect_vec().windows(2).for_each(|window| {
            if let [a,b] = *window {
                let diff = b-a;
                diffs = (diffs.1,diffs.2,diffs.3,diff);
                if !diff_sequences.contains_key(&diffs) {
                    diff_sequences.insert(diffs, *b);
                }
                all_diff_sequences.insert(diffs);
            }
        });
        diff_sequences
    }).collect_vec();
    let mut best = (0, (0,0,0,0));
    all_diff_sequences.iter().for_each(|diffs| {
        let cur = maps.iter().map(|map| {
            map.get(diffs).unwrap_or(&0)
        }).sum::<i64>();
        if cur > best.0 {
            best = (cur, *diffs);
        }
    });
    if _test {
        println!("Best result is {} with sequence {:?}", best.0, best.1);
        for (i, map) in maps.iter().enumerate() {
            println!("With value {} from start {}", map.get(&best.1).unwrap_or(&0), starts[i]);
        }
    }
    best.0 as usize
}
