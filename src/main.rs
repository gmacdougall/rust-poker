mod poker;

use std::io::{self, Read};

use crate::poker::hand::Hand;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    for line in buffer.split("\n") {
        let mut hands = vec![];
        if line.len() < 5 {
            // FIXME: Last line of file
            break;
        }
        for hand in line.split("|") {
            // FIXME: Unsafe unwrap
            hands.push(Hand::parse(hand).unwrap());
        }

        match hands.iter().max() {
            None => (),
            Some(max) => {
                // FIXME: Support ties
                println!("{}, Winner: {}", line, max);
            },
        }
    }
    Ok(())
}
