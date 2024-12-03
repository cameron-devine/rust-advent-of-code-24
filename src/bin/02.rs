advent_of_code::solution!(2);
use itertools::Itertools;
#[derive(PartialEq)]
enum SafetyFlag {
    Safe,
    Unsafe
}

fn check_report_safety(report: &[u32]) -> SafetyFlag {
    let mut safety = SafetyFlag::Safe;
    let increasing = report[0] < report[1];
    let report_windows = report.iter().tuple_windows();
    report_windows.for_each(|(a,b)| {
        if a.abs_diff(*b) == 0 || a.abs_diff(*b) > 3 || increasing != (a < b) {
            safety = SafetyFlag::Unsafe;
        }
    });

    safety
}

fn check_dampened_safety(report: &[u32]) -> SafetyFlag {
    let mut safety = SafetyFlag::Unsafe;
    for n in 0..report.len() {
        let mut report_copy = report.to_vec();
        report_copy.remove(n);
        if check_report_safety(&report_copy) == SafetyFlag::Safe {
            safety = SafetyFlag::Safe;
        };
    }
    safety
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut answer = 0;
    input.lines().for_each(|l| {
        let report: Vec<u32> = l.split_whitespace().filter_map(|x| {x.parse::<u32>().ok()}).collect();
        if check_report_safety(&report) == SafetyFlag::Safe {
            answer += 1;
        }
    });
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut answer = 0;
    input.lines().for_each(|l| {
        let report: Vec<u32> = l.split_whitespace().filter_map(|x| {x.parse::<u32>().ok()}).collect();
        if check_report_safety(&report) == SafetyFlag::Safe || check_dampened_safety(&report) == SafetyFlag::Safe {
            answer += 1;
        }
    });
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
