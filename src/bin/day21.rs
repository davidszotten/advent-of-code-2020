use anyhow::{Error, Result};
use aoc2020::dispatch;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, Clone)]
struct Entry<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for Entry<'a> {
    type Error = Error;
    fn try_from(s: &'a str) -> Result<Entry<'a>> {
        let mut it = s.split(" (contains ");
        let ingredients_list = it.next().expect("no 'contains'");
        let allergens_list = it.next().expect("no allergens").trim_matches(')');
        Ok(Entry {
            ingredients: ingredients_list.split(' ').collect(),
            allergens: allergens_list.split(", ").collect(),
        })
    }
}

fn parse(input: &str) -> Result<Vec<Entry>> {
    input
        .split('\n')
        .map(|l| Entry::try_from(l))
        .collect::<Result<Vec<_>>>()
}

fn get_allergen_map(input: &str) -> Result<HashMap<&str, &str>> {
    let entries = parse(input)?;
    let mut potential_allergen_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for entry in entries {
        for allergen in entry.allergens {
            let allergen_entry_set: HashSet<&str> = entry.ingredients.iter().cloned().collect();
            if let Some(allergen_set) = potential_allergen_map.remove(allergen) {
                potential_allergen_map.insert(
                    allergen,
                    allergen_set
                        .intersection(&allergen_entry_set)
                        .cloned()
                        .collect(),
                );
            } else {
                potential_allergen_map.insert(allergen, allergen_entry_set);
            }
        }
    }

    // dbg!(&potential_allergen_map);

    let mut allergen_map: HashMap<&str, &str> = HashMap::new();
    let mut used = HashSet::new();
    while allergen_map.len() < potential_allergen_map.len() {
        for (allergen, ingredients) in &potential_allergen_map {
            if allergen_map.contains_key(allergen) {
                continue;
            }
            let ingredients: Vec<_> = ingredients.difference(&used).cloned().collect();
            if ingredients.len() == 1 {
                let ingredient = ingredients
                    .iter()
                    .next()
                    .expect("exactly one entry")
                    .clone();
                used.insert(ingredient);
                allergen_map.insert(ingredient, allergen);
            }
        }
    }
    Ok(allergen_map)
}

fn part1(input: &str) -> Result<usize> {
    let entries = parse(input)?;
    let allergen_map = get_allergen_map(input)?;

    Ok(entries
        .iter()
        .map(|entry| {
            entry
                .ingredients
                .iter()
                .filter(|&ingredient| !allergen_map.contains_key(ingredient))
                .count()
        })
        .sum())
}

fn part2(input: &str) -> Result<String> {
    let allergen_map = get_allergen_map(input)?;
    // dbg!(&allergen_map);
    let mut items = allergen_map.iter().collect::<Vec<_>>();
    items.sort_by_key(|t| t.1);
    let ingredients = items.iter().map(|t| t.0.clone()).collect::<Vec<_>>();
    let res = ingredients.join(",");
    Ok(res.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 5);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, "mxmxvkd,sqjhc,fvjkl".to_string());
        Ok(())
    }
}
