use std::fmt::Display;

use aoc::{Challenge, Parser as ChallengeParser};
use nom::{IResult, Parser};
use parsers::ParserExt;

fn sort_five(x: [u8; 5]) -> [u8; 5] {
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
    // a<b<d<e, c<e
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

fn joker_hand(kind: Kind, cards: [u8; 5]) -> u32 {
    let cards = cards.map(|x| if x == 11 { 1 } else { x });
    let sorted_cards = sort_five(cards);
    let joker_count = match sorted_cards {
        [_, _, _, _, 1] => 5,
        [_, _, _, 1, _] => 4,
        [_, _, 1, _, _] => 3,
        [_, 1, _, _, _] => 2,
        [1, _, _, _, _] => 1,
        _ => 0,
    };

    let kind = match (joker_count, kind) {
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

    value_of_cards(kind, cards)
}

fn parse_card(x: u8) -> u8 {
    match x {
        b'A' => 14,
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        x => x & 0xf,
    }
}

#[derive(Clone, Copy, Debug)]
enum Kind {
    High,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

fn hand_kind(sorted: [u8; 5]) -> Kind {
    let [a, b, c, d, e] = sorted;
    let compressed = (1u16 << a) | (1u16 << b) | (1u16 << c) | (1u16 << d) | (1u16 << e);

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

fn parse_cards(input: &'static str) -> IResult<&'static str, [u8; 5]> {
    // if input.is_empty() {
    //     return Err(nom::Err::Error(nom::error::Error::new(
    //         input,
    //         nom::error::ErrorKind::AlphaNumeric,
    //     )));
    // }
    let (hand, input) = input.split_at(5);
    let [a, b, c, d, e]: [u8; 5] = hand.as_bytes().try_into().unwrap();
    let cards = [
        parse_card(a),
        parse_card(b),
        parse_card(c),
        parse_card(d),
        parse_card(e),
    ];

    Ok((input, cards))
}

fn value_of_cards(kind: Kind, x: [u8; 5]) -> u32 {
    let [a, b, c, d, e] = x;
    ((kind as u32) << 20)
        + ((a as u32) << 16)
        + ((b as u32) << 12)
        + ((c as u32) << 8)
        + ((d as u32) << 4)
        + (e as u32)
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Bid {
    hand: u32,
    joker_hand: u32,
    bid: u32,
}

impl Bid {
    fn parse(input: &'static str) -> IResult<&'static str, Self> {
        let (input, cards) = parse_cards(input)?;

        let mut bid = 0;
        let mut i = 1;
        while input.as_bytes()[i] != b'\n' {
            bid *= 10;
            bid += (input.as_bytes()[i] & 0xf) as u32;
            i += 1;
        }

        let sorted_cards = sort_five(cards);
        let kind = hand_kind(sorted_cards);

        let hand = value_of_cards(kind, cards);
        let joker_hand = joker_hand(kind, cards);

        Ok((
            &input[i + 1..],
            Self {
                joker_hand,
                hand,
                bid,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution(Vec<Bid>);

impl ChallengeParser for Solution {
    fn parse(mut input: &'static str) -> IResult<&'static str, Self> {
        let mut bids = Vec::with_capacity(1000);
        while !input.is_empty() {
            let (i, bid) = Bid::parse(input)?;
            bids.push(bid);
            input = i;
        }
        Ok(("", Self(bids)))
    }
}

impl Challenge for Solution {
    const NAME: &'static str = env!("CARGO_PKG_NAME");

    fn part_one(mut self) -> impl Display {
        radsort::sort_by_key(&mut self.0, |a| a.hand);
        self.0
            .into_iter()
            .enumerate()
            .map(|(i, bid)| (i as u32 + 1) * bid.bid)
            .sum::<u32>()
    }

    fn part_two(mut self) -> impl Display {
        radsort::sort_by_key(&mut self.0, |a| a.joker_hand);
        self.0
            .into_iter()
            .enumerate()
            .map(|(i, bid)| (i as u32 + 1) * bid.bid)
            .sum::<u32>()
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
