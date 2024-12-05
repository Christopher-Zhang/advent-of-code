use itertools::Itertools;


pub async fn advent_1(data: String) -> usize {
    let mut ret = 0;
    let (updates, rules) = parse(&data);
    for update in updates {
        if check_update(&update, &rules) {
            ret += update.get(update.len() / 2).unwrap();
        }
    }
    ret
}
fn parse (data: &str) -> (Vec<Vec<usize>>, Vec<(usize,usize)>) {
    let split = data.split("\n\n").collect_vec();
    (
        split[1].split("\n").map(|update| {
            update.split(",").map(|page| page.parse::<usize>().unwrap()).collect_vec()
        }).collect_vec(),
        split[0].split("\n").map(|rule| {
            rule.split("|").map(|page| page.parse::<usize>().unwrap()).collect_tuple().unwrap()
        }).collect_vec()
    )
}
fn check_rule (update: &Vec<usize>, rule: &(usize,usize)) -> bool {
    let a = match update.iter().position(|&x| x == rule.0) {
        Some(x) => x,
        None => return true
    };
    let b = match update.iter().position(|&x| x == rule.1){
        Some(x) => x,
        None => return true
    };
    a < b
}
fn check_update (update: &Vec<usize>, rules: &Vec<(usize,usize)>) -> bool {
    for rule in rules.iter() {
        if !check_rule(&update, &rule) {
            return false;
        }
    }
    true
}

pub async fn advent_2(data: String) -> usize {
    let mut ret = 0;
    let (mut updates, rules) = parse(&data);
    for mut update in updates.iter_mut() {
        if check_update(&update, &rules) {
            continue;
        }
        while !check_update(&update, &rules) {
            for rule in rules.iter() {
                fix_order(&mut update, &rule);
            }
        }
        ret += update.get(update.len() / 2).unwrap();
    }
    ret
}
fn fix_order(update: &mut Vec<usize>, rule: &(usize,usize)) -> bool {
    let a = match update.iter().position(|&x| x == rule.0) {
        Some(x) => x,
        None => return false
    };
    let b = match update.iter().position(|&x| x == rule.1){
        Some(x) => x,
        None => return false
    };
    if a > b {
        update.swap(a, b);
        return true;
    }
    false
}
