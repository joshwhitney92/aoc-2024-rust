use anyhow::Context;
use solutions::day_2::*;
// use solutions::day_1::Solution1;
// use solutions::{day_1::Solution2};
use solutions::open_file;

pub mod solutions;

fn main() -> anyhow::Result<()> {
    // Day 1 solution.
    // let file_path = "./problem_inputs/day_1.txt";
    // let result = Solution1::solve(file_path)?;
    // let result2 =  Solution2::solve(file_path)?;

    // println!("Solution 1 Total is: {}", result);
    // println!("Solution 2 Total is: {}", result2);

    let contents = open_file("./problem_inputs/day_2.txt").context("Could not open file!")?;
    let file_data = helpers::parse_file_data(contents)?;
    let s = Solution::new();
    // let result = s.solve(&file_data)?;
    println!("Solution 1 total is: {} safe reports.", s.solve(&file_data)?);
    println!("Solution 2 total is: {} safe reports.", s.solve_with_dampener(&file_data)?);

    Ok(())
}
