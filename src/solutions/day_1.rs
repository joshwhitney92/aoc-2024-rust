// You haven't even left yet and the group of Elvish Senior Historians has
// already hit a problem: their list of locations to check is currently empty.
// Eventually, someone decides that the best place to check first would be the
// Chief Historian's office.
//
// Upon pouring into the office, everyone confirms that the Chief Historian is
// indeed nowhere to be found. Instead, the Elves discover an assortment of
// notes and lists of historically significant locations! This seems to be the
// planning the Chief Historian was doing before he left. Perhaps these notes
// can be used to determine which locations to search?
//
// Throughout the Chief's office, the historically significant locations are
// listed not by name but by a unique number called the location ID. To make
// sure they don't miss anything, The Historians split into two groups, each
// searching the office and trying to create their own complete list of location
// IDs.
//
// There's just one problem: by holding the two lists up side by side (your
// puzzle input), it quickly becomes clear that the lists aren't very similar.
// Maybe you can help The Historians reconcile their lists?
//
// For example:
//
// 3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3
//
// Maybe the lists are only off by a small amount! To find out, pair up the
// numbers and measure how far apart they are. Pair up the smallest number in
// the left list with the smallest number in the right list, then the
// second-smallest left number with the second-smallest right number, and so on.
//
// Within each pair, figure out how far apart the two numbers are; you'll need
// to add up all of those distances. For example, if you pair up a 3 from the
// left list with a 7 from the right list, the distance apart is 4; if you pair
// up a 9 with a 3, the distance apart is 6.
//
// In the example list above, the pairs and distances would be as follows:
//
//     The smallest number in the left list is 1, and the smallest number in the right list is 3. The distance between them is 2.
//     The second-smallest number in the left list is 2, and the second-smallest number in the right list is another 3. The distance between them is 1.
//     The third-smallest number in both lists is 3, so the distance between them is 0.
//     The next numbers to pair up are 3 and 4, a distance of 1.
//     The fifth-smallest numbers in each list are 3 and 5, a distance of 2.
//     Finally, the largest number in the left list is 4, while the largest number in the right list is 9; these are a distance 5 apart.
//
// To find the total distance between the left list and the right list, add up
// the distances between all of the pairs you found. In the example above, this
// is 2 + 1 + 0 + 1 + 2 + 5, a total distance of 11!

use anyhow::Context;
use regex::Regex;
use sorted_vec::SortedVec;
use std::io::BufRead;

/// Implementation of Solution 1 for Day 1.
pub struct Solution1 {}

impl Solution1 {
    pub fn solve(file_path: &str) -> anyhow::Result<i32> {
        // Get the file contents
        let parsed_data = parse_file(file_path)?;

        Ok(parsed_data
            .column_a
            .iter()
            .zip(parsed_data.column_b.iter())
            .map(|(first, second)| (first - second).abs())
            .sum())
    }
}

pub struct Solution2 {}

/// Implementation for Solution 2 of Day 1.
impl Solution2 {
    pub fn solve(file_path: &str) -> anyhow::Result<i32> {
        // Get the file contents
        let file_contents = parse_file(file_path)?;

        Ok(file_contents
            .column_a
            .iter()
            .map(|x| file_contents
                .column_b
                .iter()
                .filter(|&y| y == x).count() as i32 * x)
            .sum())
    }
}

#[derive(Debug)]
/// Represents the data file for day 1 challenge.
pub struct ParsedData {
    pub column_a: SortedVec<i32>,
    pub column_b: SortedVec<i32>,
}

/// Parse the file into `ParsedData` type.
pub fn parse_file(file_path: &str) -> anyhow::Result<ParsedData> {
    // Get the file contents
    let file_contents = super::open_file(file_path).context("Could not read the file!")?;

    // Define the regex to remove the extra `\s` in between the columns
    let re = Regex::new(r"\s+").context("Error compliing the regex")?;

    // Use a BTreeSet so i can keep the columns sorted as i add items
    let mut column_a: SortedVec<i32> = SortedVec::new();
    let mut column_b: SortedVec<i32> = SortedVec::new();

    for line in file_contents.lines() {
        match line {
            Ok(contents) => {
                // Remove the extra spaces around and in the middle of each line
                let trimmed = re.replace(&contents.trim(), " ");

                // Split the colums
                let matches: Vec<&str> = trimmed.split(' ').collect();

                column_a.insert(matches[0].parse::<i32>()?);
                column_b.insert(matches[1].parse::<i32>()?);
            }
            _ => println!("No lines left to read!"),
        }
    }

    Ok(ParsedData { column_a, column_b })
}
