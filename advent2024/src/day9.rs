use itertools::Itertools;

pub async fn advent_1(data: String) -> usize {
    let diskmap = data.chars().map(|x| x.to_digit(10).unwrap()).collect_vec();
    let mut disk: Vec<isize> = vec![];
    diskmap.iter().enumerate().for_each(|(i, length)| {
        for _ in 0..*length {
            let i = i as isize;
            if i % 2 == 0 {
                disk.push(i/2);
            }
            else {
                disk.push(-1);
            }
        }
    });
    // _print_disk(&disk);
    compact(&mut disk);
    // _print_disk(&disk);
    checksum(&disk) as usize
}

pub async fn advent_2(data: String) -> usize {
    let diskmap = data.chars().map(|x| x.to_digit(10).unwrap()).collect_vec();
    let mut disk: Vec<isize> = vec![];
    diskmap.iter().enumerate().for_each(|(i, length)| {
        for _ in 0..*length {
            let i = i as isize;
            if i % 2 == 0 {
                disk.push(i/2);
            }
            else {
                disk.push(-1);
            }
        }
    });
    // _print_disk(&disk);
    compact_full(&mut disk);
    // print_disk(&disk);
    checksum(&disk) as usize
}

fn _print_disk(disk: &Vec<isize>) {
    disk.iter().for_each(|x| {
        if *x == -1 {
            print!(".");
        }
        else {
            print!("{x}");
        }
    });
    println!();
}

fn compact(disk: &mut Vec<isize>) {
    let mut left = 0;
    let mut right = disk.len()-1;
    while left < right {
        if disk[left] >= 0 {
            left += 1;
        }
        else if disk[right] == -1 {
            right -= 1;
        }
        else {
            disk.swap(left,right);
        }
    }
}

fn compact_full(disk: &mut Vec<isize>) {
    let mut left;
    let mut right = (disk.len()-1) as isize;
    let mut file_size = 0;
    while right > 0 {
        if disk[right as usize] == -1 {
            right -= 1;
        }
        else if disk[right as usize] >= 0 {
            file_size = 0;
            let file = disk[right as usize];
            while right >= 0 && disk[right as usize] == file {
                file_size += 1;
                right -= 1;
            }
        }

        if file_size > 0 {
            // find a gap
            left = 0;
            let mut start: isize = -1;
            let mut gap = 0;
            while left < right + 2 && (left as usize) < disk.len(){
                if disk[left as usize] >= 0 {
                    left += 1;
                }
                else {
                    start = left as isize;
                    while disk[left as usize] == -1 {
                        left += 1;
                    }
                    gap = left as isize - start;
                    if gap >= file_size {
                        break;
                    }
                }
            }
            // swap if gap is big enough
            if gap >= file_size {
                for i in 0..file_size {
                    let i = i as usize;
                    disk.swap(start as usize + i, right as usize + i + 1);
                }
            }
        }
    }
}

fn checksum(disk: &Vec<isize>) -> isize {
    disk.iter().enumerate().map(|(pos,id)| {
        if *id == -1 {
            return 0;
        }
        pos as isize * *id
    }).sum()
}
