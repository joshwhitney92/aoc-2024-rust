use solutions::day_1::Solution1;

pub mod solutions;

fn main() -> anyhow::Result<()> {
    let file_path = "./problem_inputs/day_1.txt";
    let result = Solution1::solve(file_path)?;

    println!("Total is: {}", result);

    Ok(())
}
