use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

impl HandRank {
    pub fn value(&self) -> i64 {
        match *self {
            HandRank::HighCard => 0,
            HandRank::Pair => 1,
            HandRank::TwoPair => 2,
            HandRank::ThreeOfAKind => 3,
            HandRank::Straight => 4,
            HandRank::Flush => 5,
            HandRank::FullHouse => 6,
            HandRank::FourOfAKind => 7,
            HandRank::StraightFlush => 8,
        }
    }
}

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Find way to simplify
        write!(
            f,
            "{}",
            match &self {
                HandRank::HighCard => "High Card",
                HandRank::Pair => "One Pair",
                HandRank::TwoPair => "Two Pair",
                HandRank::ThreeOfAKind => "Three of a Kind",
                HandRank::Straight => "Straight",
                HandRank::Flush => "Flush",
                HandRank::FullHouse => "Full House",
                HandRank::FourOfAKind => "Four of a Kind",
                HandRank::StraightFlush => "Straight Flush",
            }
        )
    }
}

