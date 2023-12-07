use std::fmt::{self, Display};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{IResult, Parser};
use parsers::ParserExt;

type Card = u16; // 13 cards

#[derive(PartialEq, Eq, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
    sorted_cards: [Card; 5],
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct JokerHand {
    cards: [Card; 5],
    sorted_cards: [Card; 5],
}

impl From<Hand> for JokerHand {
    fn from(value: Hand) -> Self {
        let cards = value.cards;
        let cards = cards.map(|x| if x == 1 << 11 { 1 } else { x });
        let mut sorted_cards = cards;
        sorted_cards.sort();
        Self {
            cards,
            sorted_cards,
        }
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.cards {
            let value = c.trailing_zeros();
            let chars = [
                '_', '_', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
            ];
            write!(f, "{}", chars[value as usize])?;
        }
        Ok(())
    }
}

impl fmt::Debug for JokerHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.cards {
            let value = c.trailing_zeros();
            let chars = [
                'J', '_', '2', '3', '4', '5', '6', '7', '8', '9', 'T', '_', 'Q', 'K', 'A',
            ];
            write!(f, "{}", chars[value as usize])?;
        }
        Ok(())
    }
}

fn parse_card(x: u8) -> Card {
    match x {
        b'A' => 1 << 14,
        b'T' => 1 << 10,
        b'J' => 1 << 11,
        b'Q' => 1 << 12,
        b'K' => 1 << 13,
        x => 1 << (x & 0xf),
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Kind {
    High,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

impl JokerHand {
    fn kind(self) -> Kind {
        let [a, b, c, d, e] = self.sorted_cards;
        let compressed = a | b | c | d | e;

        // jokers
        let compressed = compressed & 0xfffe;

        let count = compressed.count_ones();
        if count == 0 || count == 1 {
            Kind::Five
        } else if count == 2 {
            // full house or four of a kind
            // full:
            // 22233
            // 22333
            // J2233 (22233)
            // four:
            // 23333
            // 22223
            // J2333 (23333)
            // J2223 (22223)
            // JJ223 (22223)
            // JJ233 (23333)
            // JJJ23 (22223)

            let num_jokers = if b == 1 {
                2
            } else if a == 1 {
                1
            } else {
                0
            };

            if num_jokers == 2 {
                Kind::Four
            } else if num_jokers == 1 {
                if c == d {
                    Kind::Four
                } else {
                    Kind::Full
                }
            } else if b == d {
                Kind::Four
            } else {
                Kind::Full
            }
        } else if count == 3 {
            // three of a kind or two pair
            // three:
            // 22234
            // 23334
            // 23444
            // J2234 (22234)
            // J2334 (23334)
            // J2344 (23444)
            // JJ234 (22234)
            // two pair:
            // 22334
            // 22344
            // 23344

            let has_joker = a == 1;

            if has_joker || a == c || b == d || c == e {
                Kind::Three
            } else {
                Kind::TwoPair
            }
        } else if count == 4 {
            Kind::OnePair
        } else {
            Kind::High
        }
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let kind1 = self.kind();
        let kind2 = other.kind();
        Ord::cmp(&(kind1, self.cards), &(kind2, other.cards))
    }
}

impl Hand {
    fn kind(self) -> Kind {
        let [a, b, c, d, e] = self.sorted_cards;
        let compressed = a | b | c | d | e;

        let count = compressed.count_ones();
        if count == 1 {
            Kind::Five
        } else if count == 2 {
            // full house or four of a kind
            // full:
            // 22233
            // 22333
            // four:
            // 23333
            // 22223

            if b == d {
                Kind::Four
            } else {
                Kind::Full
            }
        } else if count == 3 {
            // three of a kind or two pair
            // three:
            // 22234
            // 23334
            // 23444
            // two pair:
            // 22334
            // 22344
            // 23344

            if a == c || b == d || c == e {
                Kind::Three
            } else {
                Kind::TwoPair
            }
        } else if count == 4 {
            Kind::OnePair
        } else {
            Kind::High
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let kind1 = self.kind();
        let kind2 = other.kind();
        Ord::cmp(&(kind1, self.cards), &(kind2, other.cards))
    }
}

impl Hand {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        if input.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::AlphaNumeric,
            )));
        }
        let (hand, input) = input.split_at(5);
        let hand: [u8; 5] = hand.as_bytes().try_into().unwrap();
        let cards = hand.map(parse_card);
        let mut sorted_cards = cards;
        sorted_cards.sort();

        Ok((
            input,
            Self {
                cards,
                sorted_cards,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Bid {
    hand: Hand,
    joker_hand: JokerHand,
    bid: u16,
}

impl Bid {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let (input, hand) = Hand::parse(input)?;
        let mut bid = 0;
        let mut i = 1;
        while input.as_bytes()[i] != b'\n' {
            bid *= 10;
            bid += (input.as_bytes()[i] & 0xf) as u16;
            i += 1;
        }
        Ok((
            &input[i + 1..],
            Self {
                joker_hand: hand.into(),
                hand,
                bid,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<Bid>);

impl ChallengeParser for Solution {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        Bid::parse.many1().map(Self).parse(input)
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(mut self) -> impl Display {
        self.0.sort_by_key(|a| a.hand);
        self.0
            .into_iter()
            .enumerate()
            .map(|(i, bid)| (i + 1) * (bid.bid as usize))
            .sum::<usize>()
    }

    fn part_two(mut self) -> impl Display {
        self.0.sort_by_key(|a| a.joker_hand);
        self.0
            .into_iter()
            .enumerate()
            .map(|(i, bid)| (i + 1) * (bid.bid as usize))
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use aoc::{Challenge, Parser};

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn parse() {
        let output = Solution::parse(INPUT).unwrap().1;
        println!("{output:?}");
    }

    #[test]
    fn part_one() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_one().to_string(), "6440");
    }

    #[test]
    fn part_two() {
        let output = Solution::parse(INPUT).unwrap().1;
        assert_eq!(output.part_two().to_string(), "5905");
    }
}
