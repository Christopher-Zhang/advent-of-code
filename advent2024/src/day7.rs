use itertools::Itertools;

struct Equation {
    answer: usize,
    terms: Vec<usize>
}
fn solve_recursive(target: usize, terms: &Vec<usize>, index: usize, left: usize, part2: bool) -> bool {
    if left > target {
        return false;
    }
    if index == terms.len() {
        return left == target;
    }
    if part2 {
        let mut concat = left.to_string();
        concat.push_str(&terms[index].to_string());
        if solve_recursive(target, terms, index+1, concat.parse::<usize>().unwrap(), part2) {
            return true;
        }
    }
    solve_recursive(target, terms, index + 1, left + terms[index], part2) || solve_recursive(target, terms, index + 1, left * terms[index], part2)
}

pub async fn advent_1(data: String) -> usize {
    let mut ret = 0;
    let lines = data.split("\n");

    let equations = lines.map(|line| {
        let split = line.split(": ").collect_vec();
        let answer = split[0].parse::<usize>().unwrap();
        let terms = split[1].split(" ").map(|term| term.parse::<usize>().unwrap()).collect_vec();
        return Equation {answer, terms};
    }).collect_vec();
    for equation in equations {
        if solve_recursive(equation.answer, &equation.terms, 1, equation.terms[0], false) {
            ret += equation.answer;
        }
    }
    ret
}

pub async fn advent_2(data: String) -> usize {
    let mut ret = 0;
    let lines = data.split("\n");

    let equations = lines.map(|line| {
        let split = line.split(": ").collect_vec();
        let answer = split[0].parse::<usize>().unwrap();
        let terms = split[1].split(" ").map(|term| term.parse::<usize>().unwrap()).collect_vec();
        return Equation {answer, terms};
    }).collect_vec();
    for equation in equations {
        if solve_recursive(equation.answer, &equation.terms, 1, equation.terms[0], true) {
            ret += equation.answer;
        }
    }
    ret
}
