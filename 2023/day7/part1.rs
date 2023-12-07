use std::fs::read_to_string;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug,Eq)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn from_input(input: &str) -> Self {
        let mut iter = input.split(" ");
        let hand = iter.next().expect("Input string should have first line.");
        let cards = Self::get_cards(hand);
        let hand_type = Self::get_hand_type(hand);
        let bid = iter.next().expect("Input string should have second line.").parse::<usize>().expect("Second part should be a number");
        return Hand { hand_type: hand_type, cards: cards, bid: bid };
    }

    fn get_cards(hand: &str) -> Vec<Card> {
        let mut vec = Vec::with_capacity(6);
        for c in hand.chars() {
            vec.push(Self::get_card(c));
        }
        vec
    }

    fn get_card(card: char) -> Card {
        match card {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => unreachable!(),
        }
    }

    fn get_hand_type(hand: &str) -> HandType {
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in hand.chars() {
            map.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        match map.len() {
            1 => { return HandType::FiveOfKind; },
            2 => {
                let i = *map.iter().next().unwrap().1;
                if i == 1 || i == 4 {
                    return HandType::FourOfKind;
                } else {
                    return HandType::FullHouse;
                }
            }
            3 => {
                for (_k, v) in map.iter() {
                    if *v == 3 {
                        return HandType::ThreeOfKind;
                    }
                    if *v == 2 {
                        return HandType::TwoPair;
                    }
                }
                unreachable!();
            }
            4 => { return HandType::OnePair; },
            5 => { return HandType::HighCard; },
            _ => { unreachable!(); },
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            return self.cards.cmp(&other.cards);
        }
        return self.hand_type.cmp(&other.hand_type);
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

#[allow(unused_variables)]
fn main() {
    let mut sum: usize = 0;
    //let mut deck: Vec<Hand> = read_to_string("sample_input.txt").unwrap().lines().map(|line| Hand::from_input(line)).collect();
    let mut deck: Vec<Hand> = read_to_string("input.txt").unwrap().lines().map(|line| Hand::from_input(line)).collect();
    deck.sort();
    for (i, hand) in deck.into_iter().enumerate() {
        sum += (i+1) * hand.bid;
    }
    println!("Sum : {sum}");
}
