use anyhow::{anyhow, bail, Error, Result};
use aoc2020::dispatch;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, PartialEq)]
struct Rule<'a> {
    number: usize,
    value: RuleValue<'a>,
}

impl<'a> TryFrom<&'a str> for Rule<'a> {
    type Error = Error;
    fn try_from(s: &'a str) -> Result<Rule<'a>> {
        let mut parts = s.split(": ");
        Ok(Rule {
            number: parts.next().expect("part 1").parse()?,
            value: RuleValue::try_from(parts.next().expect("part 2"))?,
        })
    }
}

#[derive(Debug, PartialEq)]
enum RuleEntry {
    Number(usize),
    Or,
}

impl TryFrom<&str> for RuleEntry {
    type Error = Error;
    fn try_from(s: &str) -> Result<RuleEntry> {
        if s == "|" {
            Ok(RuleEntry::Or)
        } else if let Ok(number) = s.parse() {
            Ok(RuleEntry::Number(number))
        } else {
            bail!("invalid rule entry: `{}`", s)
        }
    }
}

#[derive(Debug, PartialEq)]
enum RuleValue<'a> {
    Literal(&'a str),
    Combination(Vec<RuleEntry>),
}

impl<'a> TryFrom<&'a str> for RuleValue<'a> {
    type Error = Error;
    fn try_from(s: &'a str) -> Result<RuleValue<'a>> {
        if s.starts_with('"') {
            Ok(RuleValue::Literal(&s[1..s.len() - 1]))
        } else {
            // let mut numbers = s.split(' ').filter_map(|s| s.parse::<usize>().ok());
            // dbg!(numbers.clone().collect::<Vec<_>>());
            Ok(RuleValue::Combination(
                s.split(' ')
                    .map(RuleEntry::try_from)
                    .collect::<Result<Vec<_>>>()?,
            ))
        }
    }
}

// fn parse<'a>(input: &'a str) -> Result<(Vec<RuleValue<'a>>, Vec<&'a str>)> {
fn parse<'a>(input: &'a str) -> Result<usize> {
    let mut parts = input.split("\n\n");
    let raw_rules = parts.next().unwrap();
    let raw_messages = parts.next().unwrap();
    let rules = raw_rules
        .split('\n')
        .map(Rule::try_from)
        .collect::<Result<Vec<_>>>()?;
    let messages = raw_messages.split('\n').collect::<Vec<_>>();

    let rule_map: HashMap<usize, &Rule> = rules.iter().map(|r| (r.number, r)).collect();
    let mut patterns: HashMap<usize, String> = HashMap::new();
    let mut remaining: HashSet<_> = rules.iter().map(|r| r.number).collect();
    while remaining.len() > 0 {
        // dbg!(&remaining);
        for next in remaining.iter() {
            // dbg!(&next);
            let rule = rule_map.get(&next).expect("all rules are in here");
            match &rule.value {
                RuleValue::Literal(lit) => {
                    patterns.insert(rule.number, lit.to_string());
                    remaining.remove(&rule.number);
                    break;
                }
                RuleValue::Combination(subindexes) => {
                    // dbg!(&patterns);
                    if let Ok(subpatterns) = subindexes
                        .iter()
                        .map(|rv| match rv {
                            RuleEntry::Number(n) => {
                                Ok(Some(patterns.get(n).ok_or(anyhow!("{} is not ready", n))?))
                            }
                            RuleEntry::Or => Ok(None),
                        })
                        .collect::<Result<Vec<_>>>()
                    {
                        let mut pattern = "((".to_string();
                        for entry in subpatterns {
                            pattern.push_str(match entry {
                                Some(pat) => pat,
                                None => ")|(",
                            });
                        }
                        pattern.push_str("))");
                        patterns.insert(rule.number, pattern.to_string());
                        remaining.remove(&rule.number);
                        break;
                    }
                }
            }
        }
    }

    // dbg!(&patterns);
    let pattern = patterns.get(&0).expect("have all patterns now");

    let re = Regex::new(&format!("^{}$", pattern)).expect("invalid regex");
    // dbg!(messages
    // .iter()
    // .map(|m| (m, re.is_match(m)))
    // .collect::<Vec<_>>());
    Ok(messages.iter().filter(|m| re.is_match(m)).count())
}

/*
// a ((aa|bb)(ab|ba)|(ab|ba)(aa|bb)b

(a
    (
        (aa)|(bb)(ab)|(ba)
    )|(
        (ab)|(ba)(aa)|(bb)
    )
b)


*/
fn part1(input: &str) -> Result<usize> {
    parse(input)
}

fn part2(_input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 2);
        Ok(())
    }

    #[test]
    fn test_parse_rule_value() -> Result<()> {
        assert_eq!(
            RuleValue::try_from("2 3 | 3 2")?,
            RuleValue::Combination(vec![
                RuleEntry::Number(2),
                RuleEntry::Number(3),
                RuleEntry::Or,
                RuleEntry::Number(3),
                RuleEntry::Number(2),
            ],)
        );
        assert_eq!(RuleValue::try_from(r#""a""#)?, RuleValue::Literal(&"a"));
        Ok(())
    }

    #[test]
    fn test_parse_rule() -> Result<()> {
        assert_eq!(
            Rule::try_from("1: 2 3 | 3 2")?,
            Rule {
                number: 1,
                value: RuleValue::Combination(vec![
                    RuleEntry::Number(2),
                    RuleEntry::Number(3),
                    RuleEntry::Or,
                    RuleEntry::Number(3),
                    RuleEntry::Number(2),
                ])
            }
        );
        assert_eq!(
            Rule::try_from(r#"2: "a""#)?,
            Rule {
                number: 2,
                value: RuleValue::Literal(&"a")
            }
        );
        Ok(())
    }
}
