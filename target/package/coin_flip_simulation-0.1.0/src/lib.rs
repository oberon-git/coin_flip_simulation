use rand;
use std::fmt;
use std::collections::BTreeMap;

/// Runs a coin flip simulation for a specified number of iterations and flips per iteration.
pub fn run(iterations: usize, flips_per_iteration: u32) -> BTreeMap<String, f64> {
    let mut results: BTreeMap<String, f64> = BTreeMap::new();

    for _ in 0..iterations {
        let mut flips = String::new();
        for _ in 0..flips_per_iteration {
            flips.push_str(&Coin::flip().to_string());
        }
        *results.entry(flips).or_insert(0.0) += 1.0;
    }

    let iterations: f64 = iterations as f64;
    results
        .values_mut()
        .for_each(|count| *count /= iterations);

    results
}

#[derive(PartialEq)]
enum Coin {
    Heads,
    Tails,
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Coin::{Heads, Tails};

        match self {
            Heads => write!(f, "H"),
            Tails => write!(f, "T"),
        }
    }
}

impl Coin {
    fn flip() -> Coin {
        if rand::random() {
            Coin::Heads
        } else {
            Coin::Tails
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_display() {
        let h = Coin::Heads;
        let t = Coin::Tails;

        assert_eq!(
            format!("{h}{t}"), 
            "HT"
        );
    }

    #[test]
    fn test_coin_to_string() {
        let h = Coin::Heads;

        assert_eq!(
            h.to_string(),
            "H"
        );
    }

    #[test]
    fn test_coin_eq() {
        let c1 = Coin::Heads;
        let c2 = Coin::Heads;

        assert!(c1 == c2);
    }

    #[test]
    fn test_coin_neq() {
        let c1 = Coin::Heads;
        let c2 = Coin::Tails;

        assert!(c1 != c2);
    }
}



