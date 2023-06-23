use std::{env, process};

use coin_flip_simulation;

fn main() {
    let (flips_per_iteration, iterations) = parse_args();

    let result = coin_flip_simulation::run(flips_per_iteration, iterations); 
    println!("{result}");
}

fn parse_args() -> (usize, usize) {
    let mut args = env::args();
    args.next();
 
    let on_err = || {
        eprintln!("Usage: coin_flip_simulation<flips_per_iteration: positive integer> <iterations: positve integer>");
        process::exit(1);
    };
    
    let flips_per_iteration = args.next()
        .unwrap_or_else(on_err);
    
    let flips_per_iteration: Result<usize, _> = flips_per_iteration
        .trim()
        .parse();

    let flips_per_iteration = match flips_per_iteration {
        Ok(num) => num,
        Err(_) => {
            on_err();
            0
        }
    };

    let iterations = args.next()
        .unwrap_or_else(on_err);
    
    let iterations: Result<usize, _> = iterations
        .trim()
        .parse();

    let iterations = match iterations {
        Ok(num) => num,
        Err(_) => {
            on_err();
            0
        }, 
    };

    (flips_per_iteration, iterations)
}
