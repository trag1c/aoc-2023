use itertools::Itertools;

use super::Solution;

use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

const CARDS: &str = "23456789TJQKA";
const CARDS_JOKER: &str = "J23456789TQKA";

#[derive(Eq, PartialEq, Debug, Clone)]
struct Card(char, bool);

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let card_set = if self.1 { CARDS_JOKER } else { CARDS };
        card_set.find(self.0).cmp(&card_set.find(other.0))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn group(cards: &str, joker: bool) -> Vec<i32> {
    let mut hm: HashMap<char, i32> = HashMap::new();
    cards.chars().for_each(|ch| {
        hm.insert(
            ch,
            match hm.get(&ch) {
                Some(count) => count + 1,
                None => 1,
            },
        );
    });
    let mut groups = hm.values().copied().collect_vec();
    if joker && hm.contains_key(&'J') {
        let joker_value = hm.get(&'J').unwrap();
        let joker_index = groups.iter().position(|x| x == joker_value).unwrap();
        groups.remove(joker_index);

        print!("{cards} {groups:?} + {joker_value:?}   ->   ");
        match groups.iter().max() {
            Some(max_value) => {
                let max_index = groups.iter().position(|x| x == max_value).unwrap();
                groups[max_index] += joker_value;
            }
            None => groups = vec![*joker_value],
        };
        println!("{cards} {groups:?}");
    }
    groups.sort_unstable();
    groups.reverse();
    groups
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hand {
    cards: [Card; 5],
    kind: HandKind,
    bid: u32,
}

impl Hand {
    fn new(cards: &str, bid: u32, joker: bool) -> Self {
        use HandKind::*;

        let groups = group(cards, joker);

        let type_ = match groups.as_slice() {
            [5] => FiveOAK,
            [4, 1] => FourOAK,
            [3, 2] => FullHouse,
            [3, ..] => ThreeOAK,
            [2, 2, 1] => TwoPair,
            [2, ..] => OnePair,
            _ => HighCard,
        };

        let cards: [Card; 5] = cards.as_bytes()[..5]
            .iter()
            .map(|x| Card(*x as char, joker))
            .collect_vec()
            .try_into()
            .unwrap();

        Self {
            cards,
            kind: type_,
            bid,
        }
    }
}

fn parse(input: &str, joker: bool) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            Hand::new(cards, bid.parse::<u32>().unwrap(), joker)
        })
        .collect_vec()
}

fn part1(input: &str) {
    let mut hands = parse(input, false);
    hands.sort_by_key(|h| (h.kind.clone(), h.cards.clone()));
    println!(
        "{}",
        hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| (rank as u32 + 1) * hand.bid)
            .sum::<u32>()
    );
}

fn part2(input: &str) {
    let mut hands = parse(input, true);
    hands.sort_by_key(|h| (h.kind.clone(), h.cards.clone()));
    println!(
        "{}",
        hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| (rank as u32 + 1) * hand.bid)
            .sum::<u32>()
    );
}

pub const DAY7: Solution = Solution { part1, part2 };
