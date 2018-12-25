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
    fn parse(str: String) -> Result<Card, String> {
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
        let card = Card::parse(String::from("3H")).unwrap();
        assert_eq!(Rank::Three, card.rank);
        assert_eq!(Suit::Hearts, card.suit);
    }

    #[test]
    fn test_parse_kc() {
        let card = Card::parse(String::from("KC")).unwrap();
        assert_eq!(Rank::King, card.rank);
        assert_eq!(Suit::Clubs, card.suit);
    }

    #[test]
    fn test_parse_invalid_rank() {
        match Card::parse(String::from("1C")) {
            Ok(_) => assert!(false),
            Err(m) => assert_eq!(m, "Invalid Rank"),
        };
    }

    #[test]
    fn test_parse_invalid_suit() {
        match Card::parse(String::from("9P")) {
            Ok(_) => assert!(false),
            Err(m) => assert_eq!(m, "Invalid Suit"),
        };
    }

    #[test]
    fn test_parse_missing_suit() {
        match Card::parse(String::from("8")) {
            Ok(_) => assert!(false),
            Err(m) => assert_eq!(m, "No Suit Found"),
        };
    }

    #[test]
    fn test_parse_missing_everything() {
        match Card::parse(String::from("")) {
            Ok(_) => assert!(false),
            Err(m) => assert_eq!(m, "No Rank Found"),
        };
    }
}
