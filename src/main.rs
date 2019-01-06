mod poker;

use std::io;
use std::io::prelude::*;

use crate::poker::hand::Hand;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
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
            },
            Err(_) => panic!(),
        }
    }
    Ok(())
}
