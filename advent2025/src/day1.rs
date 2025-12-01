pub async fn advent_1(data: String, _test: bool) -> usize {
    let start = 50;

    let mut current = start;
    let mut answer = 0;

    data.split("\n").for_each(|line| {
        match line.get(0..1) {
            Some("L") => {current = (current - line.get(1..line.len()).unwrap().parse::<isize>().unwrap()) % 100},
            Some("R") => {current = (current + line.get(1..line.len()).unwrap().parse::<isize>().unwrap()) % 100},
            _ => panic!("panic!")
        }
        if current < 0 {
            current += 100;
        }
        if current == 0 {
            answer += 1;
        }
    });

    answer
}

pub async fn advent_2(data: String, _test: bool) -> usize {
    let start = 50;

    let mut current = start;
    let mut answer = 0;

    data.split("\n").for_each(|line| {
        let mut increment = 0;
        let prev = current;
        match line.get(0..1) {
            Some("L") => {current -= line.get(1..line.len()).unwrap().parse::<isize>().unwrap()},
            Some("R") => {current += line.get(1..line.len()).unwrap().parse::<isize>().unwrap()},
            _ => panic!("panic!")
        }
        if current < 0 {
            increment = current.abs() / 100 + 1;
            if prev == 0 {
                increment -= 1;
            }
            current %= 100;
            if current < 0 {
                current += 100;
            }
            
        }
        else if current == 100 {
            increment = 1;
            current = 0
        }
        else if current == 0 {
            increment = 1;
        }
        else if current > 100 {
            increment = current / 100;
            current %= 100;
        }

        answer += increment;

        // println!("After {line} we are at {current} moving {increment} to {answer}")
    });

    answer as usize
}
