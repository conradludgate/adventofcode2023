use std::fmt::{self, Display};

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{IResult, Parser};
use parsers::ParserExt;

type Card = u16; // 13 cards

fn sort_five(x: [u16; 5]) -> [u16; 5] {
    let [a, b, c, d, e] = x;

    let [a, b] = if a < b { [a, b] } else { [b, a] };
    let [c, d] = if c < d { [c, d] } else { [d, c] };
    // a<b, c<d

    let [a, b, c, d] = if b < d { [a, b, c, d] } else { [c, d, a, b] };
    // a<b<d, c<d

    #[allow(clippy::collapsible_else_if)]
    let [a, b, d, e] = if e < b {
        if e < a {
            [e, a, b, d]
        } else {
            [a, e, b, d]
        }
    } else {
        if e < d {
            [a, b, e, d]
        } else {
            [a, b, d, e]
        }
    };
    #[allow(clippy::collapsible_else_if)]
    let [a, b, c, d] = if c < b {
        if c < a {
            [c, a, b, d]
        } else {
            [a, c, b, d]
        }
    } else {
        if c < d {
            [a, b, c, d]
        } else {
            [a, b, d, c]
        }
    };

    [a, b, c, d, e]
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Hand {
    kind: Kind,
    cards: [Card; 5],
}

impl Hand {
    fn joker_hand(self) -> Self {
        let cards = self.cards;
        let cards = cards.map(|x| if x == 1 << 11 { 1 } else { x });
        let sorted_cards = sort_five(cards);
        let joker_count = match sorted_cards {
            [_, _, _, _, 1] => 5,
            [_, _, _, 1, _] => 4,
            [_, _, 1, _, _] => 3,
            [_, 1, _, _, _] => 2,
            [1, _, _, _, _] => 1,
            _ => 0,
        };

        let kind = match (joker_count, self.kind) {
            (0, k) => k,
            (4 | 5, _) => Kind::Five,
            (1, Kind::High) => Kind::OnePair,
            (1, Kind::OnePair) => Kind::Three,
            (1, Kind::TwoPair) => Kind::Full,
            (1, Kind::Three) => Kind::Four,
            (1, Kind::Four) => Kind::Five,
            // one pair is from the jokers
            (2, Kind::OnePair) => Kind::Three,
            (2, Kind::TwoPair) => Kind::Four,
            (2, Kind::Full) => Kind::Five,
            // the triple is from the jokers
            (3, Kind::Three) => Kind::Four,
            (3, Kind::Full) => Kind::Five,
            _ => unimplemented!(),
        };

        Self { cards, kind }
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.cards {
            let value = c.trailing_zeros();
            let chars = [
                'J', '_', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
            ];
            write!(f, "{}", chars[value as usize])?;
        }
        write!(f, " ({:?})", self.kind)?;
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

fn hand_kind(sorted: [Card; 5]) -> Kind {
    let [a, b, c, d, e] = sorted;
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
        let sorted_cards = sort_five(cards);
        let kind = hand_kind(sorted_cards);

        Ok((input, Self { kind, cards }))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Bid {
    hand: Hand,
    joker_hand: Hand,
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
                joker_hand: hand.joker_hand(),
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
