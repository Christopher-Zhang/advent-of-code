use itertools::Itertools;

pub async fn advent_1(data: String, _test: bool) -> usize {

    let (ranges, ingredients) = parse(&data);
    ingredients.iter().map(|ing| {
        if ranges.iter().any(|range| { *ing >= range.0 && *ing <= range.1 }) {
            return 1;
        }
        0
    }).sum()
}

pub async fn advent_2(data: String, _test: bool) -> usize {
    let (mut ranges, _) = parse(&data);
    let mut final_ranges = vec![ranges[0]];
    ranges.sort_by(|a,b| { a.0.cmp(&b.0) });
    for range in ranges {
        let len = final_ranges.len();
        if final_ranges[len-1].1 >= range.0 {
            final_ranges[len-1].1 = std::cmp::max(final_ranges[len-1].1, range.1);
        }
        else {
            final_ranges.push(range);
        }
    }

    final_ranges.iter().map(|range| { range.1-range.0+1 }).sum()
}

pub fn parse(data: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut ranges = vec![];
    let mut ingredients = vec![];
    
    let split = data.split("\n\n").collect_vec();
    split[0].split("\n").for_each(|range| {
        let range_split = range.split("-").collect_vec();
        ranges.push((range_split[0].parse::<usize>().unwrap(), range_split[1].parse::<usize>().unwrap()));
    });
    split[1].split("\n").for_each(|ingredient| {
        ingredients.push(ingredient.parse::<usize>().unwrap());
    });

    (ranges, ingredients)
}