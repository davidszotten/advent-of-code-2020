use anyhow::{anyhow, bail, Result};
use aoc2020::dispatch;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, Clone)]
struct Rule<'a> {
    container: &'a str,
    contains: HashMap<&'a str, usize>,
}

fn parse_line(input: &str) -> Result<Rule> {
    lazy_static! {
        static ref LINE_RE: Regex =
            // vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            Regex::new(r"^(?P<container>\w+ \w+) bags contain (?:(?P<no>no other bags)|(?P<contains>(\d+ \w+ \w+ bags?(?:, )?)+)).$")
                .expect("line regex create");

        static ref CONTAINS_RE: Regex =
            // 5 faded blue bags, 6 dotted black bags
            Regex::new(r"(?P<count>\d+) (?P<bag>\w+ \w+) bags?(?:, )?")
                .expect("contains regex create");
    }
    let line_caps = LINE_RE
        .captures(input)
        .ok_or(anyhow!("line regex mismatch: `{}`", input))?;

    let container = line_caps
        .name("container")
        .ok_or(anyhow!("container not matched"))?
        .as_str();
    Ok(if let Some(_) = line_caps.name("no") {
        Rule {
            container,
            contains: HashMap::new(),
        }
    } else if let Some(contains_line) = line_caps.name("contains") {
        let contains_line = contains_line.as_str();
        let mut contains = HashMap::new();
        for cap in CONTAINS_RE.captures_iter(contains_line) {
            let bag = cap.name("bag").ok_or(anyhow!("no bag matched"))?.as_str();
            let count: usize = cap
                .name("count")
                .ok_or(anyhow!("no count matched"))?
                .as_str()
                .parse()?;
            contains.insert(bag.into(), count);
        }
        Rule {
            container,
            contains,
        }
    } else {
        bail!("captured neither no-bags nor contents")
    })
}

fn parse(input: &str) -> Result<Vec<Rule>> {
    input.split('\n').map(parse_line).collect()
}

fn index<'a>(contains: &'a [Rule]) -> HashMap<&'a str, Rule<'a>> {
    let mut index = HashMap::new();
    for rule in contains {
        index.insert(rule.container, rule.clone());
    }
    index
}

fn inverted_index<'a>(contains: &'a [Rule]) -> HashMap<&'a str, Vec<&'a str>> {
    let mut index = HashMap::new();
    for rule in contains {
        for &bag in rule.contains.keys() {
            index
                .entry(bag)
                .or_insert(vec![])
                .push(rule.container.clone());
        }
    }
    index
}

fn part1(input: &str) -> Result<usize> {
    let bags = parse(input)?;
    let index = inverted_index(&bags);
    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back("shiny gold".into());
    while let Some(bag) = queue.pop_front() {
        if seen.contains(&bag) {
            continue;
        }
        seen.insert(bag);
        if let Some(containers) = index.get(&bag) {
            for container in containers {
                queue.push_back(container.clone());
            }
        }
    }
    Ok(seen.len() - 1)
}

fn part2(input: &str) -> Result<usize> {
    let bags = parse(input)?;
    let index = index(&bags);
    let mut total = 0;
    let mut queue: VecDeque<(usize, &str)> = VecDeque::new();
    queue.push_back((1, "shiny gold".into()));
    while let Some((count, bag)) = queue.pop_front() {
        if let Some(rule) = index.get(bag) {
            for (subbag, subcount) in rule.contains.iter() {
                queue.push_back((count * subcount, subbag));
            }
        }
        total += count;
    }
    Ok(total - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 4);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 32);
        Ok(())
    }

    #[test]
    fn test_part2b() -> Result<()> {
        assert_eq!(
            part2(
                "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            )?,
            126
        );
        Ok(())
    }
}
