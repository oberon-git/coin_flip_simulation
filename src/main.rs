use std::io;

use coin_flip_simulation;

fn main() {
    println!("How many iterations?");
    let mut iterations = String::new();
    io::stdin()
        .read_line(&mut iterations)
        .unwrap();

    let iterations = iterations
        .trim()
        .parse()
        .expect("Please enter a positive integer");

    println!("How many flips per iteration?");
    let mut flips_per_iteration = String::new();
    io::stdin()
        .read_line(&mut flips_per_iteration)
        .unwrap();

    let flips_per_iteration = flips_per_iteration
        .trim()
        .parse()
        .expect("Please enter a positve integer");

    let results = coin_flip_simulation::run(iterations, flips_per_iteration);
    
    for (k, v) in results.iter() {
        println!("{k}: {:.3}", v);
    }
}
