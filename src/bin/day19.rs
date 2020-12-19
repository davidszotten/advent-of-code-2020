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
            Ok(RuleValue::Combination(
                s.split(' ')
                    .map(RuleEntry::try_from)
                    .collect::<Result<Vec<_>>>()?,
            ))
        }
    }
}

fn parse<'a>(input: &'a str) -> Result<(HashMap<usize, String>, Vec<&'a str>)> {
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
        let prev = remaining.len();
        for next in remaining.iter() {
            let rule = rule_map.get(&next).expect("all rules are in here");
            match &rule.value {
                RuleValue::Literal(lit) => {
                    patterns.insert(rule.number, lit.to_string());
                    remaining.remove(&rule.number);
                    break;
                }
                RuleValue::Combination(subindexes) => {
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
        if prev == remaining.len() {
            break;
        }
    }

    Ok((patterns, messages))
}

fn part1(input: &str) -> Result<usize> {
    let (patterns, messages) = parse(input)?;

    let pattern = patterns.get(&0).expect("have all patterns now");

    let re = Regex::new(&format!("^{}$", pattern)).expect("invalid regex");
    Ok(messages.iter().filter(|m| re.is_match(m)).count())
}

fn part2(input: &str) -> Result<usize> {
    let (patterns, messages) = parse(input)?;
    let p42 = &patterns.get(&42).expect("have 42");
    let p31 = &patterns.get(&31).expect("have 31");

    let repeated_re = (1..10)
        .map(|n| {
            Regex::new(&format!("^({})+({}){{{}}}({}){{{}}}$", p42, p42, n, p31, n))
                .expect("invalid regex")
        })
        .collect::<Vec<_>>();
    Ok(messages
        .iter()
        .filter(|m| repeated_re.iter().any(|re| re.is_match(m)))
        .count())
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
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(
                r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#
            )?,
            12
        );
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
