use solutions::day_1::{Solution1, Solution2};

pub mod solutions;

fn main() -> anyhow::Result<()> {
    let file_path = "./problem_inputs/day_1.txt";
    let result = Solution1::solve(file_path)?;
    let result2 =  Solution2::solve(file_path)?;

    println!("Solution 1 Total is: {}", result);
    println!("Solution 2 Total is: {}", result2);
    

    Ok(())
}
