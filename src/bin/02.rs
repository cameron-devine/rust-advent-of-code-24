advent_of_code::solution!(2);
#[derive(PartialEq)]
enum SafetyFlag {
    Safe,
    Unsafe,
}

fn check_report_safety(report: &[u32]) -> SafetyFlag {
    let increasing = report[0] < report[1];

    if report
        .windows(2)
        .any(|w| w[0].abs_diff(w[1]) == 0 || w[0].abs_diff(w[1]) > 3 || increasing != (w[0] < w[1]))
    {
        SafetyFlag::Unsafe
    } else {
        SafetyFlag::Safe
    }
}

fn check_dampened_safety(report: &[u32]) -> SafetyFlag {
    if (0..report.len()).any(|n| {
        let (left, right) = report.split_at(n);
        let report_without_n = left.iter().chain(&right[1..]).copied().collect::<Vec<_>>();
        check_report_safety(&report_without_n) == SafetyFlag::Safe
    }) {
        SafetyFlag::Safe
    } else {
        SafetyFlag::Unsafe
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let answer: u32 = input
        .lines()
        .filter_map(|line| {
            let report: Vec<u32> = line
                .split_whitespace()
                .filter_map(|word| word.parse::<u32>().ok())
                .collect();

            if check_report_safety(&report) == SafetyFlag::Safe {
                Some(1) // Count safe reports as 1
            } else {
                None
            }
        })
        .sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let answer: u32 = input
        .lines()
        .filter_map(|line| {
            let report: Vec<u32> = line
                .split_whitespace()
                .filter_map(|word| word.parse::<u32>().ok())
                .collect();

            if check_report_safety(&report) == SafetyFlag::Safe
                || check_dampened_safety(&report) == SafetyFlag::Safe
            {
                Some(1) // Count safe reports as 1
            } else {
                None
            }
        })
        .sum();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
