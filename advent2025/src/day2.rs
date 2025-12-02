use std::collections::HashMap;

pub async fn advent_1(data: String, _test: bool) -> usize {
    let mut total = 0;
    let ranges = parse(data);
    ranges.iter().for_each(|(a,b)| {
        let mut i = *a;
        while i <= *b {
            let digits = digits(i);
            if digits.is_multiple_of(2) {
                let a = i / (10_usize.pow(digits/2));
                let b = i % (10_usize.pow(digits/2));
                if a == b {
                    total += i;
                }
            }
            i += 1;
        }
    });

    total
}

pub async fn advent_2(data: String, _test: bool) -> usize {
    let mut total = 0;
    let ranges = parse(data);
    ranges.iter().for_each(|(a,b)| {
        let mut i = *a;
        while i <= *b {
            if is_repeating(i) {
                total += i;
            }
            i += 1;
        }
    });

    total
}

pub fn parse(data: String) -> HashMap<usize,usize> {
    let mut ret = HashMap::new();
    data.split(",").for_each(|range| {
        let temp: Vec<&str> = range.splitn(2, "-").collect();
        ret.insert(temp[0].trim().parse::<usize>().unwrap(), temp[1].trim().parse::<usize>().unwrap());
    });

    ret
}

pub fn digits(n: usize) -> u32 {
    let mut digits = 0;
    let mut cur = n;
    while cur != 0 {
        cur /= 10;
        digits += 1;
    }

    digits
}

pub fn is_repeating(id: usize) -> bool {
    let digits = digits(id);
    for n in 1..=(digits/2) {
        if !digits.is_multiple_of(n) {
            continue;
        }

        let mut prev = id / 10_usize.pow(digits-n);
        let mut a = 0;
        while a + n <= digits {
            let subsequence = subsequence(id, a, a+n);
            if prev != subsequence {
                break;
            }
            if a + n == digits {
                return true;
            }
            prev = subsequence;
            a += n;
        }
    }

    false
}

pub fn subsequence(id: usize, a: u32, b: u32) -> usize {
    let digits = digits(id);

    ((id - id % 10_usize.pow(digits-b)) % 10_usize.pow(digits - a)) / 10_usize.pow(digits-b)
}