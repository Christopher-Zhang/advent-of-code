use itertools::Itertools;

type Grid = Vec<Vec<char>>;

pub async fn advent_1(data: String) -> usize {
    let mut ret = 0;
    let grid = data.split("\n").map(|line| {
        line.chars().collect_vec()
    }).collect_vec();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'X' {
                ret += check_xmas(&grid, x, y);
            }
        }
    }
    ret
}

fn check_xmas(grid: &Grid, x: usize, y: usize) -> usize {
    let mut count = 0;
    let xmas = ['X','M','A','S'];
    let mut index: usize;
    let dirs = [-1,0,1];
    for x_dir in dirs {
        if x_dir == -1 && x < xmas.len() - 1 {
            continue;
        }
        for y_dir in dirs {
            if y_dir == -1 && y < xmas.len() - 1 || (x_dir == 0 && y_dir == 0) {
                continue;
            }
            index = 0;
            while index < xmas.len() {
                let xp = match x_dir {
                    1 => x + index,
                    -1 => x - index,
                    _ => x
                };
                let yp = match y_dir {
                    1 => y + index,
                    -1 => y - index,
                    _ => y
                };
                if !(in_bounds(grid, xp, yp) && grid[yp][xp] == xmas[index]) {
                    break;
                }
                index += 1;
            }
            if index == xmas.len() {
                count += 1;
            }
        }
    }
    count
}
fn in_bounds(grid: &Grid, x: usize, y: usize) -> bool {
    y < grid.len() && x < grid[0].len()
}

pub async fn advent_2(data: String) -> usize {
    let mut ret = 0;
    let grid = data.split("\n").map(|line| {
        line.chars().collect_vec()
    }).collect_vec();

    for y in 1..grid.len()-1 {
        for x in 1..grid[y].len()-1 {
            if grid[y][x] == 'A' && is_x_mas(&grid, x, y){
                ret += 1;
            }
        }
    }
    ret
}

fn is_x_mas (grid: &Grid, x: usize, y: usize) -> bool {
    let topleft = grid[y-1][x-1];
    let botright = grid[y+1][x+1];

    let topright = grid[y-1][x+1];
    let botleft = grid[y+1][x-1];

    (topleft == 'M' && botright == 'S' || topleft == 'S' && botright == 'M') && (topright == 'M' && botleft == 'S' || topright == 'S' && botleft == 'M')
}
