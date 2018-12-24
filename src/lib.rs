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
    fn parse(str: String) -> Card {
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
                _ => panic!("Invalid Rank on {}", str),
            }
            None => panic!("Missing Rank on {}", str),
        };

        let suit = match split_str.next() {
            Some(char) => match char {
                'C' => Suit::Clubs,
                'D' => Suit::Diamonds,
                'H' => Suit::Hearts,
                'S' => Suit::Spades,
                _ => panic!("Invalid Suit on {}", str),
            }
            None => panic!("No Suit specified on {}", str),
        };

        Card {
            rank,
            suit,
        }
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
        let card = Card::parse(String::from("3H"));
        assert_eq!(Rank::Three, card.rank);
        assert_eq!(Suit::Hearts, card.suit);
    }

    #[test]
    fn test_parse_kc() {
        let card = Card::parse(String::from("KC"));
        assert_eq!(Rank::King, card.rank);
        assert_eq!(Suit::Clubs, card.suit);
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_rank() {
        Card::parse(String::from("1C"));
    }

    #[test]
    #[should_panic]
    fn test_parse_missing_suit() {
        Card::parse(String::from("8"));
    }

    #[test]
    #[should_panic]
    fn test_parse_missing_everything() {
        Card::parse(String::from(""));
    }
}
