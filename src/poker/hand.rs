use std::collections::HashMap;
use std::collections::HashSet;

use crate::poker::card::Card;
use crate::poker::card::Rank;
use crate::poker::card::Suit;

pub struct Hand {
    cards: Vec<Card>,
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

    pub fn is_pair(&self) -> bool {
        self.rank_sets().len() == 4
    }

    pub fn is_two_pair(&self) -> bool {
        self.rank_sets().len() == 3 && self.most_common_rank_size() == 2
    }

    pub fn is_three_of_a_kind(&self) -> bool {
        self.rank_sets().len() == 3 && self.most_common_rank_size() == 3
    }

    pub fn is_flush(&self) -> bool {
        self.is_all_same_suit()
    }

    pub fn is_straight(&self) -> bool {
        self.is_all_consecutive()
    }

    fn rank_sets(&self) -> Vec<Rank> {
        let mut ranks: Vec<Rank> = self.cards.iter().cloned().map(|c| c.rank).collect();
        ranks.sort();
        ranks.dedup();
        ranks
    }

    fn most_common_rank_size(&self) -> i32 {
        let mut frequencies = HashMap::new();

        for c in &self.cards {
            let stat = frequencies.entry(&c.rank).or_insert(0);
            *stat += 1;
        }
        frequencies.values().max().unwrap().clone()
    }

    fn is_all_consecutive(&self) -> bool {
        let ranks: Vec<Rank> = self.rank_sets();
        if ranks.len() != 5 {
            return false;
        }
        (ranks[4].value() - ranks[0].value() == 4) ||
            (ranks[3] == Rank::Five && ranks[4] == Rank::Ace)
    }

    fn is_all_same_suit(&self) -> bool {
        let suit = &self.cards[0].suit;
        self.cards.iter().all(|c| &c.suit == suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn flush_when_all_suits_the_same() {
        let hand = Hand::parse("2C 3C 6C 9C AC").unwrap();
        assert!(hand.is_flush());
    }

    #[test]
    fn not_flush_when_all_suits_not_the_same() {
        let hand = Hand::parse("2C 3C 6H 9C AC").unwrap();
        assert!(!hand.is_flush());
    }

    #[test]
    fn straight_when_all_consecutive() {
        let hand = Hand::parse("6C 3C 4C 5D 2S").unwrap();
        assert!(hand.is_straight());
    }

    #[test]
    fn wraparound_straight() {
        let hand = Hand::parse("AC 3C 4C 5D 2S").unwrap();
        assert!(hand.is_straight());
    }

    #[test]
    fn test_not_pair() {
        let hand = Hand::parse("2C JS 9C 5D 6S").unwrap();
        assert!(!hand.is_pair());
    }

    #[test]
    fn test_pair() {
        let hand = Hand::parse("2C 2S 9C 5D 6S").unwrap();
        assert!(hand.is_pair());
    }

    #[test]
    fn test_two_pair() {
        let hand = Hand::parse("2C 5S 9C 5D 9S").unwrap();
        assert!(hand.is_two_pair());
    }

    #[test]
    fn test_not_two_pair() {
        let hand = Hand::parse("5C 5S KC 5D 9S").unwrap();
        assert!(!hand.is_two_pair());
    }

    #[test]
    fn test_not_three_of_a_kind() {
        let hand = Hand::parse("2C 5S 9C 5D 9S").unwrap();
        assert!(!hand.is_three_of_a_kind());
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = Hand::parse("5C 5S KC 5D 9S").unwrap();
        assert!(hand.is_three_of_a_kind());
    }
}

