use std::env;

use coin_flip_simulation;

fn main() {
    let usage = "Usage: coin_flip_simulation <iterations: positve integer> <flips_per_iteration: positive integer>";
    let mut args = env::args();
    args.next();

    let iterations = match args.next() {
        Some(num) => num.trim().parse().expect(usage),
        None => panic!("{}", usage),
    };

    let flips_per_iteration = match args.next() {
        Some(num) => num.trim().parse().expect(usage),
        None => panic!("{}", usage),
    };
 
    let expected = coin_flip_simulation::get_expected_probability(flips_per_iteration);
    let results = coin_flip_simulation::run(iterations, flips_per_iteration);
   
    let formatted_results = coin_flip_simulation::to_json(&results, expected);
    println!("{formatted_results}");
}


