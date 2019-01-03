mod poker;

use std::io::{self, Read};

use crate::poker::hand::Hand;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    for line in buffer.split("\n") {
        if line.len() < 5 {
            // FIXME: Last line of file
            break;
        }

        // FIXME: Unsafe unwrap
        let hands: Vec<Hand> = line
            .split("|")
            .map(|h| Hand::parse(h).unwrap())
            .collect();

        match hands.iter().max() {
            None => (),
            Some(max) => {
                println!(
                    "{}, Winner: {}, Rank: {}",
                    line,
                    hands
                        .iter()
                        .filter(|h| h == &max)
                        .map(|h| h.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    max.rank(),
                );
            },
        }
    }
    Ok(())
}
