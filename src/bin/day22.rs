use anyhow::{anyhow, bail, Result};
use aoc2020::dispatch;
use std::collections::{HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

type Deck = VecDeque<usize>;

fn score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * c)
        .sum()
}

fn parse(input: &str) -> Result<(Deck, Deck)> {
    let mut sections = input.split("\n\n");
    let p1_raw = sections.next().ok_or(anyhow!("1st section missing"))?;
    let p1_cards = p1_raw
        .split('\n')
        .skip(1)
        .map(|l| l.parse().map_err(|e| anyhow!("parse failure: {}", e)))
        .collect::<Result<VecDeque<usize>>>()?;
    let p2_raw = sections.next().ok_or(anyhow!("2nd section missing"))?;
    let p2_cards = p2_raw
        .split('\n')
        .skip(1)
        .map(|l| l.parse().map_err(|e| anyhow!("parse failure: {}", e)))
        .collect::<Result<VecDeque<usize>>>()?;

    Ok((p1_cards, p2_cards))
}

fn part1(input: &str) -> Result<usize> {
    let (mut p1_cards, mut p2_cards) = parse(input)?;
    while p1_cards.len() > 0 && p2_cards.len() > 0 {
        let p1_card = p1_cards.pop_front().expect("not empty");
        let p2_card = p2_cards.pop_front().expect("not empty");
        if p1_card > p2_card {
            p1_cards.push_back(p1_card);
            p1_cards.push_back(p2_card);
        } else if p2_card > p1_card {
            p2_cards.push_back(p2_card);
            p2_cards.push_back(p1_card);
        } else {
            bail!("cards equal!")
        };
    }
    let winner = if p1_cards.len() > 0 {
        p1_cards
    } else {
        p2_cards
    };
    Ok(score(&winner))
}

fn part2(input: &str) -> Result<usize> {
    let (p1_cards, p2_cards) = parse(input)?;
    let (_, score) = game(p1_cards.clone(), p2_cards.clone(), 1);
    Ok(score)
}

enum Player {
    P1,
    P2,
}

fn game(mut p1_cards: Deck, mut p2_cards: Deck, number: usize) -> (Player, usize) {
    let mut seen = HashSet::new();
    let mut _round_number = 1;
    while p1_cards.len() > 0 && p2_cards.len() > 0 {
        // println!("\nRound {} (Game {})", _round_number, number);
        // println!("P1: {:?}", p1_cards);
        // println!("P2: {:?}", p2_cards);
        if seen.contains(&(p1_cards.clone(), p2_cards.clone())) {
            return (Player::P1, score(&p1_cards));
        }
        seen.insert((p1_cards.clone(), p2_cards.clone()));
        let p1_card = p1_cards.pop_front().expect("not empty");
        let p2_card = p2_cards.pop_front().expect("not empty");
        // println!("P1 plays {}", p1_card);
        // println!("P2 plays {}", p2_card);

        let winner = round(
            p1_card, &p1_cards, p2_card, &p2_cards, number, /*, &mut seen*/
        );
        match winner {
            Player::P1 => {
                // println!("P1 wins round {} of game {}", _round_number, number);
                p1_cards.push_back(p1_card);
                p1_cards.push_back(p2_card);
            }
            Player::P2 => {
                // println!("P2 wins round {} of game {}", _round_number, number);
                p2_cards.push_back(p2_card);
                p2_cards.push_back(p1_card);
            }
        }
        _round_number += 1;
    }
    if p1_cards.len() == 0 {
        // println!("The winner of game {} is P2", number);
        (Player::P2, score(&p2_cards))
    } else if p2_cards.len() == 0 {
        // println!("The winner of game {} is P1", number);
        (Player::P1, score(&p1_cards))
    } else {
        panic!("someone should have won by now")
    }
}

fn round(
    p1_card: usize,
    p1_cards: &Deck,
    p2_card: usize,
    p2_cards: &Deck,
    game_number: usize,
) -> Player {
    if p1_card <= p1_cards.len() && p2_card <= p2_cards.len() {
        // println!("Playing subgame");
        let p1_cards = p1_cards.clone().make_contiguous()[..p1_card]
            .iter()
            .cloned()
            .collect::<Deck>();
        let p2_cards = p2_cards.clone().make_contiguous()[..p2_card]
            .iter()
            .cloned()
            .collect::<Deck>();
        game(p1_cards, p2_cards, game_number + 1).0
    } else {
        if p1_card > p2_card {
            Player::P1
        } else if p2_card > p1_card {
            Player::P2
        } else {
            panic!("cards equal")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 306);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 291);
        Ok(())
    }
}
