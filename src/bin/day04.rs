use anyhow::{anyhow, bail, Error, Result};
use aoc2020::dispatch;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Field {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

impl TryFrom<&str> for Field {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        use Field::*;
        Ok(match s {
            "byr" => Byr,
            "iyr" => Iyr,
            "eyr" => Eyr,
            "hgt" => Hgt,
            "hcl" => Hcl,
            "ecl" => Ecl,
            "pid" => Pid,
            "cid" => Cid,
            _ => bail!("Invalid field `{}`", s),
        })
    }
}

type Passport<'a> = HashMap<Field, &'a str>;

lazy_static! {
    static ref REQUIRED: HashSet<Field> = [
        Field::Byr,
        Field::Iyr,
        Field::Eyr,
        Field::Hgt,
        Field::Hcl,
        Field::Ecl,
        Field::Pid
    ]
    .iter()
    .cloned()
    .collect();
}

fn parse_entry(raw_entry: &str) -> Result<(Field, &str)> {
    let mut split = raw_entry.split(':');
    let raw_key = split.next().ok_or(anyhow!("No key"))?;
    let key = Field::try_from(raw_key)?;
    let value = split.next().ok_or(anyhow!("No value"))?;
    if let Some(extra) = split.next() {
        bail!("Unexpected extra data: `{}`", extra);
    }
    Ok((key, value))
}

fn parse_passport(raw_passport: &str) -> Result<Passport> {
    Ok(raw_passport
        .split_whitespace()
        .map(parse_entry)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .collect::<Passport>())
}

#[derive(Debug)]
enum Unit {
    Cm,
    In,
}

impl TryFrom<&str> for Unit {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        match &s[s.len() - 2..] {
            "cm" => Ok(Unit::Cm),
            "in" => Ok(Unit::In),
            _ => bail!("Invalid unit"),
        }
    }
}

fn is_valid_entry(key: Field, value: &str) -> Result<()> {
    use Field::*;

    let int_range = |s: &str, min: i32, max: i32| -> Result<()> {
        let val: i32 = s.parse()?;
        if val < min || val > max {
            bail!("{:?} `{}` outside range", key, value);
        }
        Ok(())
    };

    Ok(match key {
        Byr => int_range(value, 1920, 2002)?,
        Iyr => int_range(value, 2010, 2020)?,
        Eyr => int_range(value, 2020, 2030)?,
        Hgt => {
            let unit = Unit::try_from(value)?;
            let amount = &value[..value.len() - 2];
            match unit {
                Unit::Cm => int_range(amount, 150, 193)?,
                Unit::In => int_range(amount, 59, 76)?,
            }
        }
        Hcl => {
            if !value.starts_with("#") {
                bail!("bad hcl start");
            }
            i64::from_str_radix(&value[1..], 16)?;
        }
        Ecl => {
            if !matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth") {
                bail!("bad hcl");
            }
        }

        Pid => {
            if value.len() != 9 {
                bail!("bad pid len");
            }
            value.trim_start_matches(|c| c == '0').parse::<i64>()?;
        }
        Cid => {}
    })
}

fn has_required_keys(passport: &Passport) -> bool {
    let keys: HashSet<_> = passport.keys().cloned().collect();
    !REQUIRED.difference(&keys).next().is_some()
}

fn is_valid(passport: &Passport) -> bool {
    if !has_required_keys(passport) {
        return false;
    }
    let keys: HashSet<_> = passport.keys().cloned().collect();
    if REQUIRED.difference(&keys).next().is_some() {
        return false;
    }

    for (&key, value) in passport.iter() {
        if is_valid_entry(key, value).is_err() {
            return false;
        }
    }

    true
}

fn part1(input: &str) -> Result<usize> {
    Ok(input
        .split("\n\n")
        .map(parse_passport)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(has_required_keys)
        .count())
}

fn part2(input: &str) -> Result<usize> {
    Ok(input
        .split("\n\n")
        .map(parse_passport)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(is_valid)
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_split() -> Result<()> {
        assert_eq!(INPUT.split("\n\n").count(), 4);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 2);
        Ok(())
    }

    #[test]
    fn test_is_valid_entry() {
        fn valid(key: &str, value: &str) -> bool {
            let field = Field::try_from(key).expect("invalid field");
            is_valid_entry(field, value).is_ok()
        }
        assert!(valid("byr", "2002"));
        assert!(!valid("byr", "2003"));
        assert!(valid("hgt", "60in"));
        assert!(valid("hgt", "190cm"));
        assert!(!valid("hgt", "190in"));
        assert!(!valid("hgt", "190"));
        assert!(valid("hcl", "#123abc"));
        assert!(!valid("hcl", "#123abz"));
        assert!(!valid("hcl", "123abc"));
        assert!(valid("ecl", "brn"));
        assert!(!valid("ecl", "wat"));
        assert!(valid("pid", "000000001"));
        assert!(!valid("pid", "0123456789"));
    }

    #[test]
    fn test_part2_invalid() -> Result<()> {
        assert_eq!(
            part2(
                "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
            )?,
            0
        );
        Ok(())
    }

    #[test]
    fn test_part2_valid() -> Result<()> {
        assert_eq!(
            part2(
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            )?,
            4
        );
        Ok(())
    }
}
