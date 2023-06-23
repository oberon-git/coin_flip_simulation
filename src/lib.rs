//! Libary for a coin flipping simulation.
//!
//! Calculates the expected and empirical counts and probabilities of coin flips for a specified
//! number of iterations and flips per iteration.

use rand;
use std::fmt;
use std::collections::BTreeMap;

/// Runs a coin flip simulation for a specified number of iterations and flips per iteration.
///
/// # Examples
/// ```
/// use coin_flip_simulation;
///
/// let result = coin_flip_simulation::run(3, 8000);
///
/// assert_eq!(result.iterations, 8000);
/// assert_eq!(result.expected.count, 1000);
/// assert_eq!(result.expected.probability, 0.125f64);
/// assert!(result.results.get("HHH").is_some());
/// ```
pub fn run(flips_per_iteration: usize, iterations: usize) -> CoinFlipResult {
    let outcomes = get_all_outcomes(flips_per_iteration);
    let mut results: BTreeMap<String, usize> = outcomes
        .into_iter()
        .map(|outcome| (outcome, 0))
        .collect();

    for _ in 0..iterations {
        let mut flips = String::new();
        for _ in 0..flips_per_iteration {
            flips.push_str(&Coin::flip().to_string());
        }
        *results.entry(flips).or_insert(0) += 1;
    }

    let results = results
        .into_iter()
        .map(|(key, count)|{
            (key, EmpiricalResult::new(count, iterations))
        })
        .collect();
    
    let expected = EmpiricalResult::expected(flips_per_iteration, iterations);
    CoinFlipResult::new(iterations, expected, results)
}

/// Gets a vector of all possible outcomes as strings.
///
/// # Examples
/// ```
/// use coin_flip_simulation;
///
/// let outcomes = coin_flip_simulation::get_all_outcomes(3);
///
/// assert_eq!(outcomes, vec!["HHH", "HHT", "HTH", "HTT", "THH", "THT", "TTH", "TTT"]);
/// ```
pub fn get_all_outcomes(flips_per_iteration: usize) -> Vec<String> {
    let num_outcomes = get_num_outcomes(flips_per_iteration);
    let mut results = Vec::with_capacity(num_outcomes);
    
    let base: usize = 2;
    for i in 0..num_outcomes {
        let mut outcome = String::with_capacity(flips_per_iteration);
        for j in 1..=flips_per_iteration {
            let d = num_outcomes / base.pow(j as u32);
            outcome.push_str(
                if (i / d) % base == 0 { "H" } else { "T" }
            );
        }

        results.push(outcome);
    }

    results
}

/// Gets the number of possible outcomes.
///
/// ```
/// use coin_flip_simulation;
///
/// let num_outcomes = coin_flip_simulation::get_num_outcomes(3);
///
/// assert_eq!(num_outcomes, 8);
/// ```
pub fn get_num_outcomes(flips_per_iteration: usize) -> usize {
    let base: usize = 2;
    base.pow(flips_per_iteration as u32)
}

/// Represents the result of running the coin flip simulation.
///
/// Contains the emprical results, the number of iterations, and the expected result.
pub struct CoinFlipResult {
    pub iterations: usize,
    pub expected: EmpiricalResult,
    pub results: BTreeMap<String, EmpiricalResult>,
}

impl CoinFlipResult {
    fn new(iterations: usize, expected: EmpiricalResult, results: BTreeMap<String, EmpiricalResult>) -> Self {
        CoinFlipResult {
            iterations,
            expected,
            results,
        }
    }
    
    /// Converts a CoinFlipResult to a json string.
    pub fn to_json_string(&self) -> String {    
        let indent = "    ";
        let mut json = format!(
            "{{\n{indent}iterations: {}\n{indent}expected: {:.5}\n{indent}actual: {{\n", 
            self.iterations, 
            self.expected,
        );

        for (k, v) in self.results.iter() {
            let entry = format!(
                "{indent}{indent}{k}: {v}\n"
            );
            json.push_str(&entry);
        }
    
        let close = format!("{indent}}}\n}}");
        json.push_str(&close);
        json
    }
}

impl fmt::Display for CoinFlipResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",  self.to_json_string())
    }
}

/// Represents an empirical result.
///
/// Contains the raw count of an outcome and the observed probability.
pub struct EmpiricalResult {
    pub count: usize,
    pub probability: f64,
}

impl EmpiricalResult {
    fn new(count: usize, iterations: usize) -> Self {
        let probability: f64 = (count as f64) / (iterations as f64);

        EmpiricalResult {
            count,
            probability,
        }
    }

    /// Gets the expected result for a specified number of iterations and flips_per_iteration.
    pub fn expected(flips_per_iteration: usize, iterations: usize) -> Self {
        let num_outcomes = get_num_outcomes(flips_per_iteration);
        let count = iterations / num_outcomes;

        EmpiricalResult::new(count, iterations)
    }
}

impl fmt::Display for EmpiricalResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{count: {}, probability: {:.5}}}", self.count, self.probability)
    }
}

#[derive(PartialEq)]
enum Coin {
    Heads,
    Tails,
}

impl Coin {
    fn flip() -> Self {
        if rand::random() {
            Coin::Heads
        } else {
            Coin::Tails
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_outcomes_4() {
        use Coin::{Heads, Tails};

        let outcomes = get_all_outcomes(4);

        let mut expected = Vec::new();
        for i in [Heads, Tails] {
            for j in [Heads, Tails] {
                for k in [Heads, Tails] {
                    for l in [Heads, Tails] {
                        expected.push(format!("{i}{j}{k}{l}"));
                    }
                }
            }
        }

        assert_eq!(outcomes, expected);
    }

    #[test]
    fn test_get_all_outcomes_6() {
        use Coin::{Heads, Tails};

        let outcomes = get_all_outcomes(6);

        let mut expected = Vec::new();
        for i in [Heads, Tails] {
            for j in [Heads, Tails] {
                for k in [Heads, Tails] {
                    for l in [Heads, Tails] {
                        for m in [Heads, Tails] {
                            for n in [Heads, Tails] {
                                expected.push(format!("{i}{j}{k}{l}{m}{n}"));
                            }
                        }
                    }
                }
            }
        }

        assert_eq!(outcomes, expected);
    }

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

