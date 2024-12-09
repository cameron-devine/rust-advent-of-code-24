use std::str::FromStr;
use crate::Operations::{Add, Concact, Multiply};

advent_of_code::solution!(7);

#[derive(Debug)]
struct EquationParseError;
#[derive(Debug)]
struct EquationSolverError;

#[derive(Debug, PartialEq, Eq)]
enum Operations {
    Add,
    Multiply,
    Concact
}
#[derive(Debug)]
struct Equation {
    test_value: u64,
    operators: Vec<u64>,
    operations: Vec<Operations>,
}

impl Equation {
    fn new(test_value: u64, operators: Vec<u64>) -> Equation {
        Equation {
            test_value,
            operators,
            operations: vec![],
        }
    }
    fn add_operators(&mut self, operators: Vec<Operations>) {
        self.operations = operators;
    }

    fn solve(&self) -> Result<bool, EquationSolverError> {
        let mut value_holder = vec![self.operators.first().unwrap().clone()];
        for operator in self.operators.iter().skip(1) {
            value_holder = value_holder
                .iter()
                .map(|x| {
                    let mut int_solution: Vec<u64> = vec![];
                    for op in self.operations.iter() {
                        match op {
                            Add => int_solution.push(x+operator),
                            Multiply => int_solution.push(x*operator),
                            Concact => int_solution.push(x * 10u64.pow(operator.ilog10() + 1) + operator),
                        }
                    }
                    int_solution
                })
                .collect::<Vec<Vec<_>>>()
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
        }
        Ok(value_holder.contains(&self.test_value))
    }
}

impl FromStr for Equation {
    type Err = EquationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(":").collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(EquationParseError);
        }
        let test_value: u64 = split[0].parse().unwrap();
        let operators: Vec<u64> = split[1]
            .split_whitespace()
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(Equation::new(test_value, operators))
    }
}
pub fn part_one(input: &str) -> Option<u64> {

    let mut sum = 0;
    for line in input.lines() {
        let mut equation = Equation::from_str(line).ok()?;
        equation.add_operators(vec![Add, Multiply]);
        if equation.solve().expect("Shoot") {
            sum += equation.test_value;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let mut equation = Equation::from_str(line).ok()?;
        equation.add_operators(vec![Add, Multiply, Concact]);
        if equation.solve().expect("Shoot") {
            sum += equation.test_value;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
