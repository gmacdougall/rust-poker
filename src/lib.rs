#[derive(Debug, PartialEq)]
enum Rank {
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

#[derive(Debug, PartialEq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn parse(str: &str) -> Result<Card, String> {
        let mut split_str = str.chars();

        let rank = match split_str.next() {
            Some(char) => match char {
                '2' => Rank::Two,
                '3' => Rank::Three,
                '4' => Rank::Four,
                '5' => Rank::Five,
                '6' => Rank::Six,
                '7' => Rank::Seven,
                '8' => Rank::Eight,
                '9' => Rank::Nine,
                'T' => Rank::Ten,
                'J' => Rank::Jack,
                'Q' => Rank::Queen,
                'K' => Rank::King,
                'A' => Rank::Ace,
                _ => return Err(String::from("Invalid Rank")),
            },
            None => return Err(String::from("No Rank Found")),
        };

        let suit = match split_str.next() {
            Some(char) => match char {
                'C' => Suit::Clubs,
                'D' => Suit::Diamonds,
                'H' => Suit::Hearts,
                'S' => Suit::Spades,
                _ => return Err(String::from("Invalid Suit")),
            },
            None => return Err(String::from("No Suit Found")),
        };

        Ok(
            Card {
                rank,
                suit,
            }
        )
    }
}

struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn parse(str: &str) -> Result<Hand, String> {
        // FIXME: Dangerous unwrap
        let mut iter = str.split(" ").map(|c| Card::parse(c).unwrap());

        Ok(
            Hand {
                cards: [
                    // FIXME: Dangerous unwrap
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                ]
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_works() {
        let card = Card {
            rank: Rank::Two,
            suit: Suit::Clubs,
        };
        assert_eq!(Rank::Two, card.rank);
        assert_eq!(Suit::Clubs, card.suit);
    }

    #[test]
    fn test_parse_3h() {
        let card = Card::parse("3H").unwrap();
        assert_eq!(Rank::Three, card.rank);
        assert_eq!(Suit::Hearts, card.suit);
    }

    #[test]
    fn test_parse_kc() {
        let card = Card::parse("KC").unwrap();
        assert_eq!(Rank::King, card.rank);
        assert_eq!(Suit::Clubs, card.suit);
    }

    #[test]
    fn test_parse_invalid_rank() {
        match Card::parse("1C") {
            Ok(_) => assert!(false),
            Err(m) => assert_eq!(m, "Invalid Rank"),
        };
    }

    #[test]
    fn test_parse_invalid_suit() {
        match Card::parse("9P") {
            Ok(_) => assert!(false),
            Err(m) => assert_eq!(m, "Invalid Suit"),
        };
    }

    #[test]
    fn test_parse_missing_suit() {
        match Card::parse("8") {
            Ok(_) => assert!(false),
            Err(m) => assert_eq!(m, "No Suit Found"),
        };
    }

    #[test]
    fn test_parse_missing_everything() {
        match Card::parse("") {
            Ok(_) => assert!(false),
            Err(m) => assert_eq!(m, "No Rank Found"),
        };
    }

    #[test]
    fn hand_contains_five_cards() {
        let hand = Hand {
            cards: [
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
}
