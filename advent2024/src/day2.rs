use itertools::Itertools;

pub async fn advent_1(data: String) -> usize {
    let mut ret = 0;
    let lines = data.split("\n").map(|line| {
        line.split(" ").map(|val| {val.parse::<usize>().unwrap()}).collect_vec()
    }).collect_vec();

    for line in lines {
        let iter = line.windows(2);
        let mut safe = true;
        let mut dir = 0;
        'inner: for window in iter {
            let a = window[0];
            let b = window[1];
            if a == b {
                safe = false;
                break 'inner;
            }
            if a > b {
                if a - b > 3 {
                    safe = false;
                    break 'inner;
                }
                if dir == 0 {
                    dir = 1;
                }
                if dir == -1 {
                    safe = false;
                    break 'inner;
                }
            }
            else {
                if b - a > 3 {
                    safe = false;
                    break 'inner;
                }
                if dir == 0 {
                    dir = -1;
                }
                if dir == 1 {
                    safe = false;
                    break 'inner;
                }
            }
            
        }
        if safe {
            ret += 1;
        }
    }
    ret
}

pub async fn advent_2(data: String) -> usize {
    let mut ret = 0;
    let lines = data.split("\n").map(|line| {
        line.split(" ").map(|val| {val.parse::<usize>().unwrap()}).collect_vec()
    }).collect_vec();

    for mut line in lines {
        let res = solve(&line);
        if res == 0 {
            ret += 1;
        }
        else {
            let mut new_line = line.clone();
            new_line.remove(res);
            if solve(&new_line) == 0 {
                ret += 1;
            }
            else {
                line.remove(res-1);
                if solve(&line) == 0 {
                    ret += 1;
                }
            }
        }
    }
    ret
}

fn solve(line: &Vec<usize>) -> usize {
    let mut dir = 0;
    for i in 0..line.len()-1 {
        let a = line[i];
        let b = line[i+1];
        if a == b {
            return i+1;
        }
        if a > b {
            if a - b > 3 {
                return i+1;
            }
            if dir == 0 {
                dir = 1;
            }
            if dir == -1 {
                return i+1;
            }
        }
        else {
            if b - a > 3 {
                return i+1;
            }
            if dir == 0 {
                dir = -1;
            }
            if dir == 1 {
                return i+1;
            }
        }
    }
    0
}
