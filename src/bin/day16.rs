use anyhow::{anyhow, Error, Result};
use aoc2020::dispatch;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
struct Field<'a> {
    name: &'a str,
    r1_low: usize,
    r1_high: usize,
    r2_low: usize,
    r2_high: usize,
}

impl<'a> Field<'a> {
    fn valid(&self, value: usize) -> bool {
        (value >= self.r1_low && value <= self.r1_high)
            || (value >= self.r2_low && value <= self.r2_high)
    }
}

impl<'a> TryFrom<&'a str> for Field<'a> {
    type Error = Error;
    fn try_from(s: &'a str) -> Result<Self> {
        let mut it = s.split(": ");
        let name = it.next().ok_or(anyhow!("no field name"))?;
        let ranges = it.next().ok_or(anyhow!("no ranges"))?;

        let mut it = ranges.split(" or ");
        let range1 = it.next().ok_or(anyhow!("no first range"))?;
        let range2 = it.next().ok_or(anyhow!("no second range"))?;

        let mut it = range1.split('-');
        let r1_low = it.next().ok_or(anyhow!("r1_low missing"))?.parse()?;
        let r1_high = it.next().ok_or(anyhow!("r1_high missing"))?.parse()?;
        let mut it = range2.split('-');
        let r2_low = it.next().ok_or(anyhow!("r2_low missing"))?.parse()?;
        let r2_high = it.next().ok_or(anyhow!("r2_high missing"))?.parse()?;
        Ok(Field {
            name,
            r1_low,
            r1_high,
            r2_low,
            r2_high,
        })
    }
}

fn parse_ticket(s: &str) -> Result<Ticket> {
    s.split(',')
        .map(|s| s.parse().map_err(|e| anyhow!("parse failure: {}", e)))
        .collect::<Result<Ticket>>()
}

type Ticket = Vec<usize>;

fn ticket_valid(ticket: &Ticket, fields: &[Field]) -> bool {
    ticket
        .iter()
        .all(|&value| fields.iter().any(|field| field.valid(value)))
}

fn parse(input: &str) -> Result<(Vec<Field>, Ticket, Vec<Ticket>)> {
    let mut parts = input.split("\n\n");
    let field_constraints = parts.next().unwrap();
    let my_ticket_raw = parts.next().unwrap();
    let nearby_tickets_raw = parts.next().unwrap();

    let fields = field_constraints
        .split('\n')
        .map(|s| Field::try_from(s))
        .collect::<Result<Vec<_>>>()?;
    let my_ticket = parse_ticket(
        my_ticket_raw
            .split('\n')
            .skip(1)
            .next()
            .ok_or(anyhow!("not enough lines"))?,
    )?;
    let nearby_tickets = nearby_tickets_raw
        .split('\n')
        .skip(1)
        .map(|s| parse_ticket(s))
        .collect::<Result<Vec<_>>>()?;

    Ok((fields, my_ticket, nearby_tickets))
}

fn field_map<'a>(
    fields: &'a [Field],
    ticket: Ticket,
    valid_tickets: &[Ticket],
) -> HashMap<&'a str, usize> {
    let mut field_pos: HashMap<&str, usize> = HashMap::new();
    let mut field_values = vec![vec![]; ticket.len()];
    for ticket in valid_tickets {
        for (index, value) in ticket.iter().enumerate() {
            field_values[index].push(value);
        }
    }
    let mut remaining_fields: HashSet<_> = fields.iter().collect();
    let mut remaining_positions: HashSet<_> = (0..ticket.len()).collect();
    while &remaining_positions.len() > &0 {
        for position in &remaining_positions.clone() {
            let values: Vec<_> = valid_tickets.iter().map(|v| v[*position]).collect();
            let possible_fields: Vec<_> = remaining_fields
                .iter()
                .filter(|f| values.iter().all(|v| f.valid(*v)))
                .collect();
            if possible_fields.len() == 1 {
                let field = possible_fields[0].clone();
                field_pos.insert(&field.name, *position);
                remaining_fields.remove(field);
                remaining_positions.remove(position);
                break;
            }
        }
    }
    field_pos.iter().map(|(k, v)| (*k, ticket[*v])).collect()
}

fn part1(input: &str) -> Result<usize> {
    let (fields, _, nearby_tickets) = parse(input)?;
    let it = nearby_tickets.iter().flat_map(|t| t.iter());
    Ok(it
        .filter(|&value| !fields.iter().any(|field| field.valid(*value)))
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    let (fields, ticket, nearby_tickets) = parse(input)?;
    let valid_tickets: Vec<_> = nearby_tickets
        .into_iter()
        .filter(|t| ticket_valid(t, &fields))
        .collect();
    let fields = field_map(&fields, ticket, &valid_tickets);
    let departure: Vec<_> = fields
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .collect();
    assert_eq!(departure.len(), 6);
    Ok(departure.iter().map(|(_, &v)| v).product())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_parse() -> Result<()> {
        // no panic
        parse(INPUT)?;
        Ok(())
    }

    #[test]
    fn test_parse_field() -> Result<()> {
        let field = Field::try_from("row: 6-11 or 33-44")?;
        assert_eq!(
            field,
            Field {
                name: "row".into(),
                r1_low: 6,
                r1_high: 11,
                r2_low: 33,
                r2_high: 44,
            }
        );
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 71);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let (fields, ticket, nearby_tickets) = parse(input)?;
        assert_eq!(
            field_map(&fields, ticket, &nearby_tickets),
            [
                ("class".into(), 12),
                ("row".into(), 11),
                ("seat".into(), 13)
            ]
            .iter()
            .cloned()
            .collect()
        );
        Ok(())
    }
}
