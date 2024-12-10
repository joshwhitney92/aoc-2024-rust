// --- Day 2: Red-Nosed Reports ---
//
// Fortunately, the first location The Historians want to search isn't a long
// walk from the Chief Historian's office.
//
// While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain
// no sign of the Chief Historian, the engineers there run up to you as soon as
// they see you. Apparently, they still talk about the time Rudolph was saved
// through molecular synthesis from a single electron.
//
// They're quick to add that - since you're already here - they'd really
// appreciate your help analyzing some unusual data from the Red-Nosed reactor.
// You turn to check if The Historians are waiting for you, but they seem to
// have already divided into groups that are currently searching every corner of
// the facility. You offer to help with the unusual data.
//
// The unusual data (your puzzle input) consists of many reports, one report per
// line. Each report is a list of numbers called levels that are separated by
// spaces. For example:
//
// 7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9
//
// This example data contains six reports each containing five levels.
//
// The engineers are trying to figure out which reports are safe. The Red-Nosed
// reactor safety systems can only tolerate levels that are either gradually
// increasing or gradually decreasing. So, a report only counts as safe if both
// of the following are true:
//
//     The levels are either all increasing or all decreasing.
//     Any two adjacent levels differ by at least one and at most three.
//
// In the example above, the reports can be found safe or unsafe by checking those rules:
//
//     7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
//     1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
//     9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
//     1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
//     8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
//     1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
//
// So, in this example, 2 reports are safe.
//
// Analyze the unusual data from the engineers. How many reports are safe?
//

use helpers::FileData;

pub mod helpers;

pub struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Self {}
    }

    pub fn is_safe_with_damper(&self, report: &Vec<i32>) -> bool {
        for (index, _) in report.iter().enumerate() {
            let modified: Vec<i32> = report
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != index)
                .map(|(_, value)| *value)
                .collect();

            if self.is_safe(&modified) {
                return true;
            }
        }
        return false;
    }

    pub fn is_safe(&self, report: &Vec<i32>) -> bool {
        let mut direction: Option<i32> = None;

        let mut i = 1;
        while i < report.len() {
            let diff = report[i] - report[i - 1];

            // Differences must be between [1, 3]
            if diff < -3 || diff > 3 || diff == 0 {
                return false;
            }

            // Check the direction
            let current_direction = if diff > 0 { 1 } else { -1 };
            match direction {
                Some(d) => {
                    if current_direction != d {
                        return false;
                    }
                }
                None => {
                    direction = Some(current_direction);
                }
            }

            i += 1;
        }

        return true;
    }

    pub fn solve_with_dampener(&self, file_data: &FileData) -> anyhow::Result<i32> {
        let mut safe_count = 0;
        for report in &file_data.reports {
            if self.is_safe(&report) || self.is_safe_with_damper(&report) {
                safe_count += 1;
            }
        }

        Ok(safe_count)
    }

    pub fn solve(&self, file_data: &FileData) -> anyhow::Result<i32> {
        let mut safe_count = 0;
        for report in &file_data.reports {
            if self.is_safe(&report) {
                safe_count += 1;
            }
        }

        Ok(safe_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        assert!(true);
    }
}
