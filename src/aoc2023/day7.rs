use std::collections::HashMap;

use itertools::Itertools;
use smallvec::SmallVec;

use lib::{aoc, challenge::Challenge};

pub struct Day7;

impl Challenge for Day7 {
    aoc!(year = 2023, day = 7);

    fn solve(input: String) -> (String, String) {
        let card_value: HashMap<char, u8> = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ]
        .into_iter()
        .zip(2..)
        .collect();

        let hands = input
            .lines()
            .map(|line| {
                let (cards, bid) = line.split_once(' ').unwrap();
                let cards: SmallVec<[u8; 5]> = cards
                    .chars()
                    .map(|c| *card_value.get(&c).unwrap())
                    .collect();

                (cards, bid.parse::<usize>().unwrap())
            })
            .collect_vec();

        let fst = hands
            .iter()
            .map(|(cards, bid)| (hand_strength(cards, 0), bid))
            .sorted_by_key(|h| h.0)
            .zip(1..)
            .map(|((_, bid), rank)| bid * rank)
            .sum::<usize>();

        let joker = *card_value.get(&'J').unwrap();
        let snd = hands
            .into_iter()
            .map(|(cards, bid)| (hand_strength(&cards, joker), bid))
            .sorted_by_key(|h| h.0)
            .zip(1..)
            .map(|((_, bid), rank)| bid * rank)
            .sum::<usize>();

        (fst.to_string(), snd.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    Nothing,
}

impl Hand {
    fn value(&self) -> usize {
        match self {
            Hand::FiveOfAKind => 10,
            Hand::FourOfAKind => 9,
            Hand::FullHouse => 8,
            Hand::ThreeOfAKind => 7,
            Hand::TwoPair => 6,
            Hand::OnePair => 5,
            Hand::Nothing => 1,
        }
    }

    fn apply_jokers(&mut self, jokers: u8) {
        *self = match self {
            Hand::FiveOfAKind => Hand::FiveOfAKind,
            Hand::FourOfAKind => Hand::FiveOfAKind,
            Hand::FullHouse => Hand::FiveOfAKind,
            Hand::ThreeOfAKind => Hand::FourOfAKind,
            Hand::TwoPair => {
                if jokers > 1 {
                    Hand::FourOfAKind
                } else {
                    Hand::FullHouse
                }
            }
            Hand::OnePair => Hand::ThreeOfAKind,
            Hand::Nothing => Hand::OnePair,
        }
    }
}

fn hand_strength(cards: &SmallVec<[u8; 5]>, joker: u8) -> usize {
    let groups: SmallVec<[u8; 5]> = cards
        .iter()
        .sorted()
        .chunk_by(|&c| c)
        .into_iter()
        .map(|g| g.1.count() as u8)
        .sorted()
        .rev()
        .collect();

    let mut hand_type = match *groups.as_slice() {
        [5] => Hand::FiveOfAKind,
        [4, 1] => Hand::FourOfAKind,
        [3, 2] => Hand::FullHouse,
        [3, 1, 1] => Hand::ThreeOfAKind,
        [2, 2, 1] => Hand::TwoPair,
        [2, 1, 1, 1] => Hand::OnePair,
        [1, 1, 1, 1, 1] => Hand::Nothing,
        _ => unreachable!(),
    };

    let jokers = cards.iter().filter(|c| **c == joker).count();
    if jokers > 0 {
        hand_type.apply_jokers(jokers as u8);
    }

    cards
        .into_iter()
        .map(|&card| if card == joker { 0 } else { card as usize })
        .fold(hand_type.value(), |acc, card| acc * 15 + card)
}
