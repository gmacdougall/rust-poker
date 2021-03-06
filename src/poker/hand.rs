use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

use crate::poker::card::Card;
use crate::poker::hand_rank::HandRank;
use crate::poker::rank::Rank;

pub struct Hand {
    cards: Vec<Card>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        self.rank().value().cmp(
            &other.rank().value()
        ).then(
            self.value().cmp(&other.value())
        )
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.value() == other.value() &&
            self.rank() == other.rank()
    }
}

impl Eq for Hand {}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.cards
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}

impl Hand {
    pub fn parse(str: &str) -> Result<Hand, String> {
        let vec: Vec<Card> = match str.split(" ").map(|c| Card::parse(c)).collect() {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        if vec.len() != 5 {
            return Err(String::from("Wrong Length!"));
        }

        Ok(Hand { cards: vec })
    }

    pub fn value(&self) -> [i64; 9] {
        let mut four_rank: i64 = 0;
        let mut three_rank: i64 = 0;
        let mut pairs = vec![];
        let mut singles = vec![];

        for (key, val) in self.rank_sizes() {
            if val == 4 {
                four_rank = key.value();
            } else if val == 3 {
                three_rank = key.value();
            } else if val == 2 {
                pairs.push(key.value());
            } else {
                singles.push(key.value());
            }
        }

        pairs.sort();
        singles.sort();

        if self.is_wraparound_straight() {
            return [0, 0, 0, 0, 5, 4, 3, 2, 1];
        }

        [
            four_rank,
            three_rank,
            *pairs.get(1).unwrap_or(&0),
            *pairs.get(0).unwrap_or(&0),
            *singles.get(4).unwrap_or(&0),
            *singles.get(3).unwrap_or(&0),
            *singles.get(2).unwrap_or(&0),
            *singles.get(1).unwrap_or(&0),
            *singles.get(0).unwrap_or(&0)
        ]
    }

    pub fn rank(&self) -> HandRank {
        if self.is_straight_flush() {
            HandRank::StraightFlush
        } else if self.is_four_of_a_kind() {
            HandRank::FourOfAKind
        } else if self.is_full_house() {
            HandRank::FullHouse
        } else if self.is_flush() {
            HandRank::Flush
        } else if self.is_straight() {
            HandRank::Straight
        } else if self.is_three_of_a_kind() {
            HandRank::ThreeOfAKind
        } else if self.is_two_pair() {
            HandRank::TwoPair
        } else if self.is_pair() {
            HandRank::Pair
        } else {
            HandRank::HighCard
        }
    }

    fn is_pair(&self) -> bool {
        self.rank_sets().len() == 4
    }

    fn is_two_pair(&self) -> bool {
        self.rank_sets().len() == 3 && self.most_common_rank_size() == 2
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.rank_sets().len() == 3 && self.most_common_rank_size() == 3
    }

    fn is_straight(&self) -> bool {
        self.is_all_consecutive() || self.is_wraparound_straight()
    }

    fn is_flush(&self) -> bool {
        self.is_all_same_suit()
    }

    fn is_full_house(&self) -> bool {
        self.rank_sets().len() == 2 && self.most_common_rank_size() == 3
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.rank_sets().len() == 2 && self.most_common_rank_size() == 4
    }

    fn is_straight_flush(&self) -> bool {
        self.is_straight() && self.is_flush()
    }

    fn rank_sets(&self) -> Vec<&Rank> {
        let mut ranks: Vec<&Rank> = self.cards.iter().map(|c| &c.rank).collect();
        ranks.sort();
        ranks.dedup();
        ranks
    }

    fn rank_sizes(&self) -> HashMap<&Rank, i64> {
        let mut frequencies = HashMap::new();

        for c in &self.cards {
            let stat = frequencies.entry(&c.rank).or_insert(0);
            *stat += 1;
        }
        frequencies
    }

    fn most_common_rank_size(&self) -> i64 {
        *self.rank_sizes().values().max().unwrap()
    }

    fn is_wraparound_straight(&self) -> bool {
        let ranks: Vec<&Rank> = self.rank_sets();

        ranks.len() == 5 && (*ranks[3] == Rank::Five && *ranks[4] == Rank::Ace)
    }

    fn is_all_consecutive(&self) -> bool {
        let ranks: Vec<&Rank> = self.rank_sets();

        ranks.len() == 5 && (ranks[4].value() - ranks[0].value() == 4)
    }

    fn is_all_same_suit(&self) -> bool {
        self.cards.iter().all(|c| &c.suit == &self.cards[0].suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poker::suit::Suit;

    #[test]
    fn hand_contains_five_cards() {
        let hand = Hand {
            cards: vec![
                Card::parse("2C").unwrap(),
                Card::parse("2D").unwrap(),
                Card::parse("6C").unwrap(),
                Card::parse("9H").unwrap(),
                Card::parse("AS").unwrap(),
            ]
        };

        assert_eq!(Rank::Two, hand.cards[0].rank);
        assert_eq!(Rank::Two, hand.cards[1].rank);
        assert_eq!(Rank::Six, hand.cards[2].rank);
        assert_eq!(Rank::Nine, hand.cards[3].rank);
        assert_eq!(Rank::Ace, hand.cards[4].rank);

        assert_eq!(Suit::Clubs, hand.cards[0].suit);
        assert_eq!(Suit::Diamonds, hand.cards[1].suit);
        assert_eq!(Suit::Clubs, hand.cards[2].suit);
        assert_eq!(Suit::Hearts, hand.cards[3].suit);
        assert_eq!(Suit::Spades, hand.cards[4].suit);
    }

    #[test]
    fn hand_parses_correctly() {
        let hand = Hand::parse("2C 2D 6C 9H AS").unwrap();

        assert_eq!(Rank::Two, hand.cards[0].rank);
        assert_eq!(Rank::Two, hand.cards[1].rank);
        assert_eq!(Rank::Six, hand.cards[2].rank);
        assert_eq!(Rank::Nine, hand.cards[3].rank);
        assert_eq!(Rank::Ace, hand.cards[4].rank);

        assert_eq!(Suit::Clubs, hand.cards[0].suit);
        assert_eq!(Suit::Diamonds, hand.cards[1].suit);
        assert_eq!(Suit::Clubs, hand.cards[2].suit);
        assert_eq!(Suit::Hearts, hand.cards[3].suit);
        assert_eq!(Suit::Spades, hand.cards[4].suit);
    }

    #[test]
    fn hand_parse_with_4_cards() {
        match Hand::parse("2C 2D 6C 9H") {
            Ok(_) => assert!(false "4 cards should not succeed"),
            Err(msg) => assert_eq!(msg, "Wrong Length!"),
        }
    }

    #[test]
    fn hand_parse_with_6_cards() {
        match Hand::parse("2C 2D 6C 9H KS KD") {
            Ok(_) => assert!(false "6 cards should not succeed"),
            Err(msg) => assert_eq!(msg, "Wrong Length!"),
        }
    }

    #[test]
    fn test_high_card() {
        let hand = Hand::parse("2C JS 9C 5D 6S").unwrap();
        assert_eq!(HandRank::HighCard, hand.rank());
    }

    #[test]
    fn test_pair() {
        let hand = Hand::parse("2C 2S 9C 5D 6S").unwrap();
        assert_eq!(HandRank::Pair, hand.rank());
    }

    #[test]
    fn test_two_pair() {
        let hand = Hand::parse("2C 5S 9C 5D 9S").unwrap();
        assert_eq!(HandRank::TwoPair, hand.rank());
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = Hand::parse("5C 5S KC 5D 9S").unwrap();
        assert_eq!(HandRank::ThreeOfAKind, hand.rank());
    }

    #[test]
    fn straight_when_all_consecutive() {
        let hand = Hand::parse("6C 3C 4C 5D 2S").unwrap();
        assert_eq!(HandRank::Straight, hand.rank());
    }

    #[test]
    fn wraparound_straight() {
        let hand = Hand::parse("AC 3C 4C 5D 2S").unwrap();
        assert_eq!(HandRank::Straight, hand.rank());
    }

    #[test]
    fn flush_when_all_suits_the_same() {
        let hand = Hand::parse("2C 3C 6C 9C AC").unwrap();
        assert_eq!(HandRank::Flush, hand.rank());
    }

    #[test]
    fn test_full_house() {
        let hand = Hand::parse("5C 5S KC 5D KS").unwrap();
        assert_eq!(HandRank::FullHouse, hand.rank());
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand = Hand::parse("5C 5S KC 5D 5H").unwrap();
        assert_eq!(HandRank::FourOfAKind, hand.rank());
    }

    #[test]
    fn test_straight_flush() {
        let hand = Hand::parse("3S 5S 4S 7S 6S").unwrap();
        assert_eq!(HandRank::StraightFlush, hand.rank());
    }

    #[test]
    fn test_wraparound_straight_flush() {
        let hand = Hand::parse("3S 5S 4S AS 2S").unwrap();
        assert_eq!(HandRank::StraightFlush, hand.rank());
    }

    #[test]
    fn test_comparison_of_different_hands() {
        let straight_flush = Hand::parse("3S 5S 4S AS 2S").unwrap();
        let four_of_a_kind = Hand::parse("5C 5S KC 5D 5H").unwrap();
        let full_house = Hand::parse("5C 5S KC 5D KS").unwrap();
        let flush = Hand::parse("2C 3C 6C 9C AC").unwrap();
        let straight = Hand::parse("6C 3C 4C 5D 2S").unwrap();
        let three_of_a_kind = Hand::parse("5C 5S KC 5D 9S").unwrap();
        let two_pair = Hand::parse("2C 5S 9C 5D 9S").unwrap();
        let pair = Hand::parse("2C 2S 9C 5D 6S").unwrap();
        let high_card = Hand::parse("2C JS 9C 5D 6S").unwrap();

        assert!(straight_flush > four_of_a_kind);
        assert!(four_of_a_kind > full_house);
        assert!(full_house > flush);
        assert!(flush > straight);
        assert!(straight > three_of_a_kind);
        assert!(three_of_a_kind > two_pair);
        assert!(two_pair > pair);
        assert!(pair > high_card);
    }

    #[test]
    fn test_comparison_of_different_straight_flushes() {
        let ace_high = Hand::parse("TC JC QC KC AC").unwrap();
        let king_high = Hand::parse("KS JS QS 9S TS").unwrap();
        let five_high = Hand::parse("AD 2D 3D 4D 5D").unwrap();

        assert!(ace_high > king_high);
        assert!(king_high > five_high);
    }

    #[test]
    fn test_comparison_of_different_straights() {
        let ace_high_straight = Hand::parse("TC JC QC KD AS").unwrap();
        let king_high_straight = Hand::parse("KC JC QC 9D TS").unwrap();
        let five_high_straight = Hand::parse("AC 2C 3C 4D 5S").unwrap();

        assert!(ace_high_straight > king_high_straight);
        assert!(king_high_straight > five_high_straight);
    }

    #[test]
    fn test_comparison_of_different_four_of_a_kind() {
        let four_tens = Hand::parse("TC TS TH TD 2S").unwrap();
        let four_nines = Hand::parse("9C 9S 9H 9D AS").unwrap();

        assert!(four_tens > four_nines);
    }

    #[test]
    fn test_comparison_of_different_full_houses() {
        let threes_full_of_kings = Hand::parse("3C 3H 3C KD KS").unwrap();
        let threes_full_of_fives = Hand::parse("3C 3H 3C 5D 5S").unwrap();
        let sixes_full_of_eights = Hand::parse("6C 6S 6H 8D 8S").unwrap();

        assert!(sixes_full_of_eights > threes_full_of_kings);
        assert!(threes_full_of_kings > threes_full_of_fives);
    }

    #[test]
    fn test_comparison_of_high_card() {
        let king_high = Hand::parse("4H 5C 9D KS JS").unwrap();
        let nine_high = Hand::parse("4H 5C 9D 6S 2S").unwrap();

        let h1 = Hand::parse("4D JH KD 2C 7D").unwrap();
        let h2 = Hand::parse("5C 4C KC 2D JC").unwrap();

        assert!(king_high > nine_high);
        assert!(h1 > h2);
    }

    #[test]
    fn test_comparison_of_flush_vs_non_flush() {
        let flush = Hand::parse("2H QH 6H KH AH").unwrap();
        let non_flush = Hand::parse("KD AS 2C 6D QS").unwrap();

        assert!(flush > non_flush);
        assert_ne!(flush, non_flush);
    }
}

