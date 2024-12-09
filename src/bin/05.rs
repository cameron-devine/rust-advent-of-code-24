use regex::Regex;
use std::collections::HashMap;
use std::ops::Index;
use std::str::FromStr;
use crate::Status::{Invalid, Unknown, Valid};
use petgraph::graph::Graph;
use petgraph::visit::IntoNodeReferences;
use petgraph::algo::toposort;
use petgraph::Directed;
use petgraph::prelude::GraphMap;
use petgraph::prelude::DiGraph;

advent_of_code::solution!(5);

#[derive(Debug, Eq, PartialEq)]
enum Status {
    Unknown,
    Valid,
    Invalid,
    Fixed
}
struct Update {
    updates: HashMap<usize, usize>,
    status: Status,
}

impl Update {
    fn set_status(&mut self, status: Status) -> () {
        self.status = status;
    }

    fn fix_order(&mut self, corrected_updates: Vec<usize>) -> () {
        let mut page_nums: HashMap<usize, usize> = HashMap::new();
        for (i, num) in corrected_updates.into_iter().enumerate() {
            page_nums.insert(num, i);
        }
        self.updates = page_nums;
        self.status = Status::Fixed;
    }

    fn middle_value(&self) -> u32 {
        let mid_index = match self.updates.len() {
            0..2 => self.updates.len(),
            _ => (self.updates.len() - 1) / 2,
        };
        let mut mid_value: u32 = 0;
        for (key, value) in self.updates.iter() {
            if *value == mid_index {
                mid_value = *key as u32;
            }
        }
        mid_value
    }
}
struct ManualUpdates {
    order_rules: HashMap<usize, Vec<usize>>,
    graph: DiGraph<usize,()>,
    updates: Vec<Update>,
}

impl ManualUpdates {
    fn get_topological_order(&self) -> Vec<usize> {
        let g = self.graph.clone();
        let sorted: Vec<usize> = toposort(&g, None).unwrap().iter().map(|i| i.index()).collect();
        sorted
    }

    fn validate_rules(&mut self) -> () {
        for u in self.updates.iter_mut() {
            let mut status = Valid;
            for (page_num, index) in u.updates.clone().iter() {
                for rule in self.order_rules.get(page_num).unwrap_or(&vec![]).iter() {
                    if u.updates.contains_key(rule) && u.updates.get(rule) < Option::from(index) {
                        status = Invalid;
                    }
                }
            }
            u.set_status(status);
        }
    }

    fn fix_rules(&mut self) -> () {
        let correct_order = self.get_topological_order();
        for u in self.updates.iter_mut().filter(|u| u.status == Invalid) {
            let mut updated_order = vec![];
            for number in correct_order.iter() {
                if u.updates.contains_key(number) {
                    updated_order.push(*number);
                }
            }
            u.fix_order(updated_order);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseManualUpdatesError;
impl FromStr for ManualUpdates {
    type Err = ParseManualUpdatesError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rule_map: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut edges = vec![];
        let mut updates_list: Vec<Update> = Vec::new();

        let ins_re = Regex::new(r"(?<a>\d+)\|(?<b>\d+)").unwrap();
        let pages_re = Regex::new(r"\d").unwrap();
        s.lines().into_iter().for_each(|line| {
            if ins_re.is_match(line) {
                let page_nums = line
                    .split("|")
                    .into_iter()
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                rule_map
                    .entry(page_nums[0])
                    .or_insert(Vec::new())
                    .push(page_nums[1]);
                edges.push((page_nums[0] as u32, page_nums[1] as u32));
            } else if pages_re.is_match(line) {
                let mut page_nums: HashMap<usize, usize> = HashMap::new();
                for (i, num) in line.split(",").enumerate() {
                    page_nums.insert(num.parse::<usize>().unwrap(), i);
                }

                updates_list.push(Update {
                    updates: page_nums,
                    status: Unknown
                });
            }
        });
        let mut manual = ManualUpdates { order_rules: rule_map, updates: updates_list, graph: DiGraph::<_, ()>::from_edges(edges) };
        ManualUpdates::validate_rules(&mut manual);
        Ok(manual)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let manual = ManualUpdates::from_str(input).unwrap();
    Some(
        manual
            .updates
            .iter()
            .filter_map(|x| match x.status {
                Valid => Some(x.middle_value()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .into_iter()
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut manual = ManualUpdates::from_str(input).unwrap();
    manual.fix_rules();
    Some(
        manual
            .updates
            .iter()
            .filter_map(|x| match x.status {
                Status::Fixed => Some(x.middle_value()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .into_iter()
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(123));
    }
}
