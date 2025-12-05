use std::time::Instant;
use std::fs;
use anyhow::Result;
use crate::day5::{advent_1, advent_2};
mod day5;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    let test = "./test";
    let path = "./input";
    let test_data = fs::read_to_string(test).unwrap();
    let input_data = fs::read_to_string(path).unwrap();

    println!("Test results: ");
    {
        let now = Instant::now();
        let result = advent_1(test_data.clone(), true).await;
        println!("Final total part 1: {:?}", result);
        let elapsed = now.elapsed();
        println!("Part 1 finished in {:.2?}", elapsed);
     
        let now = Instant::now();
        let result = advent_2(test_data, true).await;
        println!("Final total part 2: {:?}", result);
        let elapsed = now.elapsed();
        println!("Part 2 finished in {:.2?}", elapsed);
    }
    if !input_data.is_empty() {
        println!();
        println!();
        println!("Input results: ");
        let now = Instant::now();
        let result = advent_1(input_data.clone(), false).await;
        println!("Final total part 1: {:?}", result);
        let elapsed = now.elapsed();
        println!("Part 1 finished in {:.2?}", elapsed);
     
        let now = Instant::now();
        let result = advent_2(input_data, false).await;
        println!("Final total part 2: {:?}", result);
        let elapsed = now.elapsed();
        println!("Part 2 finished in {:.2?}", elapsed);
    }

    Ok(())
}
