use anyhow::Result;
use aoc2020::dispatch;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;
use pest_derive::*;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Parser)]
#[grammar = "bin/day18.pest"] // relative to src
struct Calc;

fn eval(expression: Pairs<Rule>, climber: &PrecClimber<Rule>) -> i64 {
    climber.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.as_str().parse::<i64>().unwrap(),
            Rule::expr => eval(pair.into_inner(), climber),
            _ => unreachable!(),
        },
        |lhs: i64, op: Pair<Rule>, rhs: i64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::multiply => lhs * rhs,
            _ => unreachable!(),
        },
    )
}

fn part1(input: &str) -> Result<i64> {
    use Assoc::*;
    use Rule::*;
    let climber = PrecClimber::new(vec![
        Operator::new(add, Left) | Operator::new(multiply, Left),
    ]);

    let mut sum = 0;
    for line in input.split('\n') {
        let ast = Calc::parse(Rule::calculation, line)?;
        sum += eval(ast, &climber);
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<i64> {
    use Assoc::*;
    use Rule::*;
    let climber = PrecClimber::new(vec![
        Operator::new(multiply, Left),
        Operator::new(add, Left),
    ]);

    let mut sum = 0;
    for line in input.split('\n') {
        let ast = Calc::parse(Rule::calculation, line)?;
        sum += eval(ast, &climber);
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("1 + 2 * 3 + 4 * 5 + 6")?, 71);
        assert_eq!(part1("1 + (2 * 3) + (4 * (5 + 6))")?, 51);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2("1 + 2 * 3 + 4 * 5 + 6")?, 231);
        Ok(())
    }
}
