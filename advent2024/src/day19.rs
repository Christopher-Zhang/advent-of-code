use itertools::Itertools;
use cached::proc_macro::cached;
use cached::SizedCache;


pub async fn advent_1(data: String, _test: bool) -> usize {
    let (towels, designs) = parse(&data);
    let towels = towels.iter().filter(|towel| towel.contains("g") || towel.len() == 1).map(|towel| towel.to_string()).collect_vec();
    designs.iter().map(|design| {
        if is_valid_design(&towels, &design) {
            return 1;
        }
        0
    }).sum()
}

#[cached(
    ty = "SizedCache<String, bool>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ format!("{}",design) }"#
)]
fn is_valid_design(towels: &Vec<String>, design: &str) -> bool {
    if design.len() == 0 {
        return true;
    }
    for towel in towels {
        if let Some(stripped) = design.strip_prefix(towel) {
            if is_valid_design(towels, stripped) {
                return true;
            }
        }
    }
    false
}

fn parse(data: &str) -> (Vec<String>, Vec<String>) {
    let split = data.split("\n\n").collect_vec();
    let towels = split[0].split(", ").map(|s| s.to_string()).collect_vec();
    let designs = split[1].split("\n").map(|s| s.to_string()).collect_vec();
    (towels, designs)
}

pub async fn advent_2(data: String, _test: bool) -> usize {
    let (towels, designs) = parse(&data);
    designs.iter().map(|design| {
        num_valid_designs(&towels, &design)
    }).sum()
}

#[cached(
    ty = "SizedCache<String, usize>",
    create = "{ SizedCache::with_size(100)}",
    convert = r#"{ format!("{}",design) }"#
)]
fn num_valid_designs(towels: &Vec<String>, design: &str) -> usize {
    if design.len() == 0 {
        return 1;
    }
    towels.iter().map(|towel| {
        if let Some(stripped) = design.strip_prefix(towel) {
            return num_valid_designs(towels, stripped);
        }
        0
    }).sum()
}
