use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::zip;

use crate::{DayRunner, FileLoader, Part, TaskType};

#[derive(Debug)]
pub struct Day07;

impl DayRunner for Day07 {
    fn run(part: Part) {
        let result = match part {
            Part::PartOne => part_one(&FileLoader::load("07", &TaskType::Puzzle)),
            Part::PartTwo => part_two(&FileLoader::load("07", &TaskType::Puzzle)),
        };

        Self::report_result(Self, part, result);
    }
}

fn part_one(data: &str) -> u32 {
    run(PokerHand::create_hands(data, &HandType::NonJokered))
}

fn part_two(data: &str) -> u32 {
    run(PokerHand::create_hands(data, &HandType::Jokered))
}

fn run(mut poker_hands: Vec<PokerHand>) -> u32 {
    poker_hands.sort();

    let scored_poker_hands = ScoredPokerHand::create_hands(&poker_hands);
    let winnings = scored_poker_hands
        .iter()
        .map(|hand| hand.bid * hand.score)
        .sum();

    winnings
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Clone)]
enum HandType {
    Jokered,
    NonJokered,
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone)]
enum PokerRank {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq)]
struct PokerHand {
    cards: String,
    bid: u32,
    poker_rank: PokerRank,
    hand_type: HandType,
}

impl PokerHand {
    fn create_hands(data: &str, hand_type: &HandType) -> Vec<Self> {
        let hand_data: Vec<(String, u32)> = data
            .lines()
            .map(|line| line.split_once(' ').unwrap())
            .map(|(cards, bid)| (cards.to_string(), bid.parse::<u32>().unwrap()))
            .collect();

        let mut poker_hands: Vec<Self> = vec![];

        for (cards, bid) in hand_data {
            let char_to_count: HashMap<char, u32> = Self::char_to_count(&cards.clone(), hand_type);

            let count_to_char: HashMap<u32, Vec<char>> =
                char_to_count.iter().fold(HashMap::new(), |mut acc, x| {
                    let (char, count) = x;
                    let char = *char;
                    let count = *count;

                    acc.entry(count)
                        .and_modify(|value| value.push(char))
                        .or_insert_with(|| vec![char]);

                    acc
                });

            let poker_rank = match count_to_char.keys().max().unwrap() {
                5 => PokerRank::FiveOfAKind,
                4 => PokerRank::FourOfAKind,
                3 => {
                    if count_to_char.contains_key(&2) {
                        PokerRank::FullHouse
                    } else {
                        PokerRank::ThreeOfAKind
                    }
                }
                2 => {
                    if count_to_char.get(&2).unwrap().len() == 2 {
                        PokerRank::TwoPair
                    } else {
                        PokerRank::OnePair
                    }
                }
                1 => PokerRank::HighCard,
                _ => {
                    panic!("This is impossible.")
                }
            };

            poker_hands.push(Self {
                cards,
                bid,
                poker_rank,
                hand_type: hand_type.clone(),
            });
        }

        poker_hands
    }

    fn char_to_count(cards: &str, hand_type: &HandType) -> HashMap<char, u32> {
        return match hand_type {
            HandType::Jokered => {
                let mut char_to_count: HashMap<char, u32> = HashMap::new();
                let joker_count = cards.chars().filter(|card| *card == 'J').count();

                cards.chars().for_each(|char| {
                    char_to_count
                        .entry(char)
                        .and_modify(|value| *value += 1)
                        .or_insert(1);
                });

                let char_to_jokered_count: HashMap<char, u32> =
                    char_to_count
                        .iter()
                        .fold(HashMap::new(), |mut acc, (char, count)| {
                            if *char == 'J' {
                                acc.insert(*char, *count);
                            } else {
                                // ERROR: I'M BORROWING THE JOKER MULTIPLE TIMES!!
                                acc.insert(*char, *count + u32::try_from(joker_count).unwrap());
                            }
                            acc
                        });

                char_to_jokered_count
            }

            HandType::NonJokered => {
                let mut char_to_count: HashMap<char, u32> = HashMap::new();

                cards.chars().for_each(|char| {
                    char_to_count
                        .entry(char)
                        .and_modify(|value| *value += 1)
                        .or_insert(1);
                });

                char_to_count
            }
        };
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.poker_rank.cmp(&other.poker_rank) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match self.hand_type {
                HandType::Jokered => compare_jokered(self, other),
                HandType::NonJokered => compared_non_jokered(self, other),
            },
        }
    }
}

fn compared_non_jokered(this: &PokerHand, other: &PokerHand) -> Ordering {
    let card_strength_order: HashMap<char, u32> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);

    compare(this, other, &card_strength_order)
}

fn compare_jokered(this: &PokerHand, other: &PokerHand) -> Ordering {
    let card_strength_order: HashMap<char, u32> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]);

    compare(this, other, &card_strength_order)
}

fn compare(
    this: &PokerHand,
    other: &PokerHand,
    card_strength_order: &HashMap<char, u32>,
) -> Ordering {
    // Need to look at the cards one by one to determine ordering.
    let self_cards = this.cards.chars().collect::<Vec<char>>();

    let other_cards = other.cards.chars().collect::<Vec<char>>();
    let mut index = 0;
    while self_cards[index] == other_cards[index] {
        index += 1;
    }

    let self_card_strength = card_strength_order.get(&self_cards[index]).unwrap();
    let other_card_strength = card_strength_order.get(&other_cards[index]).unwrap();

    other_card_strength.cmp(self_card_strength)
}

#[derive(Debug)]
#[allow(dead_code)]
struct ScoredPokerHand {
    cards: String,
    bid: u32,
    poker_rank: PokerRank,
    score: u32,
}

impl ScoredPokerHand {
    fn create_hands(hands: &Vec<PokerHand>) -> Vec<Self> {
        let maximum_score = hands.len();

        zip((1..=maximum_score).rev(), hands)
            .map(|(score, poker_hand)| Self {
                cards: poker_hand.cards.clone(),
                bid: poker_hand.bid,
                poker_rank: poker_hand.poker_rank.clone(),
                score: u32::try_from(score).unwrap(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example_input() {
        assert_eq!(part_one(&FileLoader::load("07", &TaskType::Example)), 6440);
    }

    #[test]
    fn part_one_puzzle_input() {
        assert_eq!(
            part_one(&FileLoader::load("07", &TaskType::Puzzle)),
            248_559_379
        );
    }

    #[test]
    fn part_two_example_input() {
        assert_eq!(part_two(&FileLoader::load("07", &TaskType::Example)), 5905);
    }

    #[ignore]
    #[test]
    fn part_two_puzzle_input() {
        assert_eq!(
            part_two(&FileLoader::load("07", &TaskType::Puzzle)),
            20_048_741
        );
    }
}
